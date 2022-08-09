# Rust实现光追周末
### 阶段进展：2022年8月9日
这几天实现了**BVH**结构，这可以加速场景求交测试，当前再使用线程池+BVH渲染同一张图片，只需要381秒（6分21秒）。
### 阶段进展：2022年8月6日
截至今天，已完成**Ray Tracing Weekend系列**第一篇的全部内容（额外实现了并行渲染）

光追周末封面图渲染结果[SPP=500, RAY_DEPTH=50, TIME_CONSUME=8 MIN]：
![image](https://user-images.githubusercontent.com/33785908/183393614-b7b30d6e-7de1-403d-aa2e-42a7f977d28c.png)
