# framework-doc-compile
framework-doc-compile by Rust

## 实现细节

通过高效的tree算法, 对proto进行ast分析, 从而快速编译文档; 那么在nodejs的编译器中会对proto进行签名/hash保存, 避免过大的运算量, 也是一个很粗糙的空间换时间的方案
