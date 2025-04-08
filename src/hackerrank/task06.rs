#include <iostream>
#include <string>

void staircase(int n) {
for (int i = 1; i <= n; ++i) {
std::cout << std::string(n - i, ' ');
std::cout << std::string(i, '#') << std::endl;
}
}

int main() {
int n;
std::cin >> n;
staircase(n);
return 0;
}
