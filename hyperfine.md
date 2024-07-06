# Hyperfine 性能检测

## 性能检测结果

1. **平均执行时间**: 0.5 毫秒
2. **时间范围**: 0.4 毫秒到 2.0 毫秒
3. **标准差**: 0.1 毫秒
4. **用户时间**: 0.8 毫秒
5. **系统时间**: 0.0 毫秒
6. **运行次数**: 1605 次

## 主要结论

- **执行速度非常快**: 程序平均执行时间仅为 0.5 毫秒，表明其性能非常高，能够在极短时间内完成任务。
- **稳定性较高**: 标准差为 0.1 毫秒，表示执行时间的波动较小，程序运行较为稳定。

## 原始测试结果
```sh
Benchmark 1: ./tiks
  Time (mean ± σ): 0.5 ms ± 0.1 ms [User: 0.8 ms, System: 0.0 ms]
  Range (min … max): 0.4 ms … 2.0 ms 1605 runs
```

## 更新后的测试结果
--wramup 10 缓存以填满的结果
```sh
Benchmark 1: ./tiks
  Time (mean ± σ):       0.6 ms ±   0.1 ms    [User: 0.8 ms, System: 0.0 ms]
  Range (min … max):     0.5 ms …   2.5 ms    2425 runs
```