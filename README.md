# Rust实现光追周末
### 阶段进展：2022年10月25日
实现了**Ray Tracing Weekend系列**第二篇以及第三篇大部分内容，添加了ply格式网格模型的渲染支持
### 阶段进展：2022年8月9日
这几天实现了**BVH**结构，这可以加速场景求交测试，当前再使用线程池+BVH渲染同一张图片，只需要126秒（588→126，提升约4.67倍）。
### 阶段进展：2022年8月6日
截至今天，已完成**Ray Tracing Weekend系列**第一篇的全部内容（额外实现了并行渲染）

---
渲染效果图片
![image](https://user-images.githubusercontent.com/33785908/183393614-b7b30d6e-7de1-403d-aa2e-42a7f977d28c.png)![scene1](https://user-images.githubusercontent.com/48388820/197756488-dec085db-a399-42c4-932f-7799e1a8e7a2.jpg)
![scene2](https://user-images.githubusercontent.com/48388820/197756539-f5310efa-7c46-4d71-a210-cb096f1a67ca.png)
![image](https://user-images.githubusercontent.com/48388820/197756655-06bf71bb-47dd-4859-9396-0d4552461034.png)
