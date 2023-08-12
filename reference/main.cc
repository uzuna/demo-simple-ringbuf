// Reference from くまぎさんの徐々に高度になるリングバッファの話
// blog: https://kumagi.hatenablog.com/entry/ring-buffer
// gist: https://gist.github.com/kumagi/519d48b0b22eba5b4b20c62d9f82b433
#include <assert.h>
#include <atomic>
#include <chrono>
#include <cstddef>
#include <cstdint>
#include <iomanip>
#include <iostream>
#include <mutex>
#include <queue>
#include <thread>
#define _XOPEN_SOURCE_EXTENDED 1
#include <sys/mman.h>
#include <vector>

class StdQueue {
public:
  explicit StdQueue() {}
  bool enqueue(int item) {
    buffer_.push(item);
    return true;
  }
  bool dequeue(int *dest) {
    if (buffer_.empty()) {
      return false;
    }
    *dest = buffer_.front();
    return true;
  }

private:
  std::queue<int> buffer_;
};

class RingBuffer0 {
public:
  explicit RingBuffer0(size_t size) : buffer_(size) {}

  // Returns true on success. Fails if the buffer is empty.
  bool enqueue(int item) {
    if (write_idx_ - read_idx_ == buffer_.size()) {
      return false;
    }
    buffer_[write_idx_ % buffer_.size()] = item;
    write_idx_++;
    return true;
  }

  // Returns true on success. Fails if the buffer is full.
  bool dequeue(int *dest) {
    if (write_idx_ == read_idx_) {
      return false;
    }
    *dest = buffer_[read_idx_ % buffer_.size()];
    read_idx_++;
    return true;
  }

private:
  std::vector<int> buffer_;
  uint64_t read_idx_{0};
  uint64_t write_idx_{0};
};

class RingBuffer1 {
public:
  explicit RingBuffer1(size_t size) : buffer_(size) {}

  // Returns true on success. Fails if the buffer is empty.
  bool enqueue(int item) {
    if (write_idx_ - read_idx_ == buffer_.size()) {
      return false;
    }
    buffer_[write_idx_ & (buffer_.size() - 1)] = item;
    write_idx_++;
    return true;
  }

  // Returns true on success. Fails if the buffer is full.
  bool dequeue(int *dest) {
    if (write_idx_ == read_idx_) {
      return false;
    }
    *dest = buffer_[read_idx_ & (buffer_.size() - 1)];
    read_idx_++;
    return true;
  }

private:
  std::vector<int> buffer_;
  uint64_t read_idx_{0};
  uint64_t write_idx_{0};
};

class RingBufferMutex {
public:
  explicit RingBufferMutex(size_t size) : buffer_(size) {}

  // Returns true on success. Fails if the buffer is empty.
  bool enqueue(int item) {
    std::scoped_lock lock(mutex_);
    if (write_idx_ - read_idx_ == buffer_.size()) {
      return false;
    }
    buffer_[write_idx_ & (buffer_.size() - 1)] = item;
    write_idx_++;
    return true;
  }

  // Returns true on success. Fails if the buffer is full.
  bool dequeue(int *dest) {
    std::scoped_lock lock(mutex_);
    if (write_idx_ == read_idx_) {
      return false;
    }
    *dest = buffer_[read_idx_ & (buffer_.size() - 1)];
    read_idx_++;
    return true;
  }

private:
  std::mutex mutex_;
  std::vector<int> buffer_;
  uint64_t read_idx_{0};
  uint64_t write_idx_{0};
};

class RingBuffer2 {
public:
  explicit RingBuffer2(size_t size) : buffer_(size) {}

  // Returns true on success. Fails if the buffer is empty.
  bool enqueue(int item) {
    uint64_t write_idx = write_idx_.load(std::memory_order_relaxed);
    uint64_t read_idx = read_idx_.load(std::memory_order_acquire);
    if (write_idx - read_idx == buffer_.size()) {
      return false;
    }
    buffer_[write_idx & (buffer_.size() - 1)] = item;
    write_idx_.store(write_idx + 1, std::memory_order_release);
    return true;
  }

  // Returns true on success. Fails if the buffer is full.
  bool dequeue(int *dest) {
    uint64_t read_idx = read_idx_.load(std::memory_order_relaxed);
    uint64_t write_idx = write_idx_.load(std::memory_order_acquire);
    if (write_idx == read_idx) {
      return false;
    }
    *dest = buffer_[read_idx & (buffer_.size() - 1)];
    read_idx_.store(read_idx_ + 1, std::memory_order_release);
    return true;
  }

private:
  std::vector<int> buffer_;
  alignas(64) std::atomic<uint64_t> read_idx_{0};
  alignas(64) std::atomic<uint64_t> write_idx_{0};
};

class RingBuffer3 {
public:
  explicit RingBuffer3(size_t size) : buffer_(size) {}

  // Returns true on success. Fails if the buffer is empty.
  bool enqueue(int item) {
    uint64_t write_idx = write_idx_.load(std::memory_order_relaxed);
    if (write_idx - cached_read_idx_ == buffer_.size()) {
      cached_read_idx_ = read_idx_.load(std::memory_order_acquire);
      assert(cached_read_idx_ <= write_idx);
      if (write_idx - cached_read_idx_ == buffer_.size()) {
        return false;
      }
    }
    buffer_[write_idx & (buffer_.size() - 1)] = item;
    write_idx_.store(write_idx + 1, std::memory_order_release);
    return true;
  }

  // Returns true on success. Fails if the buffer is full.
  bool dequeue(int *dest) {
    uint64_t read_idx = read_idx_.load(std::memory_order_relaxed);
    if (cached_write_idx_ == read_idx) {
      cached_write_idx_ = write_idx_.load(std::memory_order_acquire);
      assert(read_idx <= cached_write_idx_);
      if (cached_write_idx_ == read_idx) {
        return false;
      }
    }
    *dest = buffer_[read_idx & (buffer_.size() - 1)];
    read_idx_.store(read_idx_ + 1, std::memory_order_release);
    return true;
  }

private:
  std::vector<int> buffer_;
  alignas(64) std::atomic<uint64_t> read_idx_{0};
  alignas(64) uint64_t cached_read_idx_{0};
  alignas(64) std::atomic<uint64_t> write_idx_{0};
  alignas(64) uint64_t cached_write_idx_{0};
};

class RingBuffer4 {
public:
  explicit RingBuffer4(size_t size)
      : buffer_((int *)mmap(0, size * sizeof(int), PROT_READ | PROT_WRITE,
                            MAP_PRIVATE | MAP_ANONYMOUS | MAP_HUGETLB, 0, 0)),
        size_(size) {}
  ~RingBuffer4() { munmap(buffer_, size_ * sizeof(int)); }

  // Returns true on success. Fails if the buffer is empty.
  bool enqueue(int item) {
    uint64_t write_idx = write_idx_.load(std::memory_order_relaxed);
    if (write_idx - cached_read_idx_ == size_) {
      cached_read_idx_ = read_idx_.load(std::memory_order_acquire);
      assert(cached_read_idx_ <= write_idx);
      if (write_idx - cached_read_idx_ == size_) {
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
        return false;
      }
    }
    buffer_[write_idx & (size_ - 1)] = item;
    write_idx_.store(write_idx + 1, std::memory_order_release);
    return true;
  }

  // Returns true on success. Fails if the buffer is full.
  bool dequeue(int *dest) {
    uint64_t read_idx = read_idx_.load(std::memory_order_release);
    if (cached_write_idx_ == read_idx) {
      cached_write_idx_ = write_idx_.load(std::memory_order_acquire);
      assert(read_idx <= cached_write_idx_);
      if (cached_write_idx_ == read_idx) {
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
        return false;
      }
    }
    *dest = buffer_[read_idx & (size_ - 1)];
    read_idx_.store(read_idx_ + 1, std::memory_order_release);
    return true;
  }

private:
  int *buffer_;
  size_t size_;
  alignas(64) std::atomic<uint64_t> read_idx_{0};
  alignas(64) uint64_t cached_read_idx_{0};
  alignas(64) std::atomic<uint64_t> write_idx_{0};
  alignas(64) uint64_t cached_write_idx_{0};
};

#define ASSERT_TRUE(x)                                                         \
  {                                                                            \
    if (!(x)) {                                                                \
      std::cerr << "Assertion fail: " #x << " at " << __LINE__ << "\n";        \
    }                                                                          \
  }

template <typename RingBufferType> void test() {
  RingBufferType rb(4);
  int result;
  ASSERT_TRUE(!rb.dequeue(&result));
  ASSERT_TRUE(rb.enqueue(1));
  ASSERT_TRUE(rb.enqueue(2));
  ASSERT_TRUE(rb.enqueue(3));
  ASSERT_TRUE(rb.enqueue(4));
  ASSERT_TRUE(!rb.enqueue(5));
  ASSERT_TRUE(rb.dequeue(&result));
  ASSERT_TRUE(result == 1);
  ASSERT_TRUE(rb.dequeue(&result));
  ASSERT_TRUE(result == 2);
  ASSERT_TRUE(rb.dequeue(&result));
  ASSERT_TRUE(result == 3);
  ASSERT_TRUE(rb.dequeue(&result));
  ASSERT_TRUE(result == 4);
  ASSERT_TRUE(!rb.dequeue(&result));
}

constexpr uint64_t kCount = 500000;

template <typename RingBufferType> double benchmark_single(RingBufferType &rb) {
  auto start = std::chrono::system_clock::now();
  int result;
  for (uint64_t i = 0; i < kCount; ++i) {
    for (int j = 0; j < 1000; ++j) {
      rb.enqueue(j);
    }
    for (int j = 0; j < 1000; ++j) {
      rb.dequeue(&result);
    }
  }
  auto end = std::chrono::system_clock::now();
  double duration =
      std::chrono::duration_cast<std::chrono::milliseconds>(end - start)
          .count();
  const int count = kCount * (1000 + 1000);
  std::cerr << count << " ops in " << duration << " ms \t";
  return count / duration;
}

template <typename RingBufferType> double benchmark(RingBufferType &rb) {
  auto start = std::chrono::system_clock::now();
  std::thread workers[2] = {
      std::thread([&]() {
        cpu_set_t cpuset;
        CPU_ZERO(&cpuset);
        CPU_SET(0, &cpuset);
        if (pthread_setaffinity_np(pthread_self(), sizeof(cpu_set_t),
                                   &cpuset) == -1) {
          perror("pthread_setaffinity_no");
          exit(1);
        }
        for (uint64_t i = 0; i < kCount; ++i) {
          int count = 1000;
          while (0 < count) {
            if (rb.enqueue(count)) {
              count--;
            }
          }
        }
      }),
      std::thread([&]() {
        cpu_set_t cpuset;
        CPU_ZERO(&cpuset);
        CPU_SET(1, &cpuset);
        if (pthread_setaffinity_np(pthread_self(), sizeof(cpu_set_t),
                                   &cpuset) == -1) {
          perror("pthread_setaffinity_no");
          exit(1);
        }
        int result;
        for (uint64_t i = 0; i < kCount; ++i) {
          int count = 1000;
          while (0 < count) {
            if (rb.dequeue(&result)) {
              count--;
            }
          }
        }
      })};
  for (auto &w : workers) {
    w.join();
  }
  auto end = std::chrono::system_clock::now();
  double duration =
      std::chrono::duration_cast<std::chrono::milliseconds>(end - start).count();
  const int count = kCount * (1000 + 1000);
  std::cerr << count << " ops in " << duration << " ms \t";
  return count / duration;
}

int main() {
  test<RingBuffer1>();
  test<RingBuffer2>();
  test<RingBuffer3>();
  StdQueue queue;
  RingBuffer0 rb0(2 * 1024 * 1024);
  RingBuffer1 rb1(2 * 1024 * 1024);
  RingBufferMutex rbm(2 * 1024 * 1024);
  RingBuffer2 rb2(2 * 1024 * 1024);
  RingBuffer3 rb3(2 * 1024 * 1024);
  RingBuffer4 rb4(2 * 1024 * 1024);
  std::cout << std::setprecision(10);
  std::cout << "StdQueue_single: " << benchmark_single(queue) << " ops/ms\n";
  std::cout << "RingBufferMutex: " << benchmark_single(rbm) << " ops/ms\n";
  std::cout << "RingBuffer0_single: " << benchmark_single(rb0) << " ops/ms\n";
  std::cout << "RingBuffer1_single: " << benchmark_single(rb1) << " ops/ms\n";
  std::cout << "RingBuffer2_single: " << benchmark_single(rb2) << " ops/ms\n";
  std::cout << "RingBuffer3_single: " << benchmark_single(rb3) << " ops/ms\n";
  std::cout << "RingBufferMutex: " << benchmark(rbm) << " ops/ms\n";
  std::cout << "RingBuffer2: " << benchmark(rb2) << " ops/ms\n";
  std::cout << "RingBuffer3: " << benchmark(rb3) << " ops/ms\n";
  std::cout << "RingBuffer4: " << benchmark(rb4) << " ops/ms\n";
}
