#include <iostream>
#include <openssl/err.h>
#include <openssl/pem.h>
#include <openssl/rsa.h>
#include <string>

// 加密函数
std::string rsa_encrypt(const std::string &plaintext, RSA *rsa_public_key) {
  int rsa_len = RSA_size(rsa_public_key);
  unsigned char *encrypted = new unsigned char[rsa_len];
  int ret = RSA_public_encrypt(
      plaintext.size(),
      reinterpret_cast<const unsigned char *>(plaintext.c_str()), encrypted,
      rsa_public_key, RSA_PKCS1_PADDING);
  if (ret == -1) {
    std::cerr << "RSA 加密失败" << std::endl;
    ERR_print_errors_fp(stderr);
    delete[] encrypted;
    return "";
  }
  std::string ciphertext(reinterpret_cast<char *>(encrypted), ret);
  delete[] encrypted;
  return ciphertext;
}

// 解密函数
std::string rsa_decrypt(const std::string &ciphertext, RSA *rsa_private_key) {
  int rsa_len = RSA_size(rsa_private_key);
  unsigned char *decrypted = new unsigned char[rsa_len];
  int ret = RSA_private_decrypt(
      ciphertext.size(),
      reinterpret_cast<const unsigned char *>(ciphertext.c_str()), decrypted,
      rsa_private_key, RSA_PKCS1_PADDING);
  if (ret == -1) {
    std::cerr << "RSA 解密失败" << std::endl;
    ERR_print_errors_fp(stderr);
    delete[] decrypted;
    return "";
  }
  std::string plaintext(reinterpret_cast<char *>(decrypted), ret);
  delete[] decrypted;
  return plaintext;
}

int main() {
  // 生成 RSA 密钥对
  RSA *rsa = RSA_generate_key(2048, RSA_F4, NULL, NULL);
  if (rsa == nullptr) {
    std::cerr << "生成 RSA 密钥对失败" << std::endl;
    return 1;
  }

  // 待加密的明文
  std::string plaintext = "Hello, world!";
  std::cout << "明文: " << plaintext << std::endl;

  // 加密
  std::string ciphertext = rsa_encrypt(plaintext, rsa);
  std::cout << "加密后的密文: " << ciphertext << std::endl;

  // 解密
  std::string decrypted_plaintext = rsa_decrypt(ciphertext, rsa);
  std::cout << "解密后的明文: " << decrypted_plaintext << std::endl;

  // 释放 RSA 密钥
  RSA_free(rsa);

  return 0;
}
