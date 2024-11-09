// 包含输入输出流库，用于使用std::cout进行控制台输出
#include <iostream>
// 包含内存管理库，提供智能指针如std::shared_ptr的支持
#include <memory>

int main()
{
  // 创建一个裸指针，初始化为nullptr，表示不指向任何有效的内存地址
  int* rawPtr = nullptr;

  {  // 开始一个新的作用域，用于限定smartPtr的生存期
    // 创建智能指针smartPtr，指向一个值为55的int对象，
    // std::指的是shared_ptr是C++标准库中提供的智能指针
    std::shared_ptr<int> smartPtr = std::make_shared<int>(55);
    rawPtr = smartPtr.get();  // 获取智能指针所管理的裸指针

    std::cout << "智能指针管理的值: " << *smartPtr << std::endl;
    std::cout << "裸指针指向的值: " << *rawPtr << std::endl;
  }  // smartPtr 在此作用域结束后被销毁，管理的内存被释放

  // 此时 rawPtr 成为悬垂指针
  // 未定义行为，在不同环境运行可能看到不同的值，也可能崩溃
  std::cout << "尝试访问悬垂裸指针的值: " << *rawPtr << std::endl;

  return 0;
}
// 运行输出:
// 智能指针管理的值: 55
// 裸指针指向的值: 55
// 尝试访问悬垂裸指针的值: 0
