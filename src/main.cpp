#include <fstream>
#include <iostream>
#include <string>

int main() {
  std::string filename = "example.txt";

  // 写文件
  std::ofstream outFile(filename);
  if (!outFile) {
    std::cerr << "无法打开文件：" << filename << std::endl;
    return 1;
  }

  outFile << "这是一个用于演示文件读写的例子。\n";
  outFile << "Hello, World!\n";
  outFile.close();

  // 读文件
  std::ifstream inFile(filename);
  if (!inFile) {
    std::cerr << "无法打开文件：" << filename << std::endl;
    return 1;
  }

  std::string line;
  while (std::getline(inFile, line)) {
    std::cout << line << std::endl;
  }
  inFile.close();

  return 0;
}
