#pragma once

#include <memory>

#include "snappy-cxx-rs/snappy/snappy-sinksource.h"
#include "snappy-cxx-rs/snappy/snappy.h"
#include "snappy-cxx-rs/src/source_sink.rs.h"

namespace snappy {

class VecSink : public snappy::Sink {
public:
  explicit VecSink(RustVecSink &sink) : m_sink(sink) {}

  void Append(const char *bytes, size_t n) override {
    return m_sink.append(bytes, n);
  }

private:
  RustVecSink &m_sink;
};

class SliceSource : public snappy::Source {
public:
  explicit SliceSource(RustSliceSource &source) : m_source(source) {}

  size_t Available() const override { return m_source.available(); }

  const char *Peek(size_t *len) override { return m_source.peek(*len); }

  void Skip(size_t n) override { return m_source.skip(n); }

private:
  RustSliceSource &m_source;
};

static inline size_t compress_source_to_sink(RustSliceSource &source,
                                             RustVecSink &sink) {
  SliceSource slice_source(source);
  VecSink vec_sink(sink);

  return snappy::Compress(&slice_source, &vec_sink);
}

static inline bool uncompress_source_to_sink(RustSliceSource &compressed,
                                             RustVecSink &uncompressed) {
  SliceSource slice_source(compressed);
  VecSink vec_sink(uncompressed);

  return snappy::Uncompress(&slice_source, &vec_sink);
}
} // namespace snappy