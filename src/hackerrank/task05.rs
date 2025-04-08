#include <iostream>
#include <iomanip>
#include <vector>

void plusMinus(const std::vector<int>& arr) {
int positive = 0, negative = 0, zero = 0;
int n = arr.size();

for (int num : arr) {
if (num > 0) positive++;
else if (num < 0) negative++;
else zero++;
}

std::cout << std::fixed << std::setprecision(6);
std::cout << static_cast<double>(positive) / n << std::endl;
std::cout << static_cast<double>(negative) / n << std::endl;
std::cout << static_cast<double>(zero) / n << std::endl;
}

int main() {
int n;
std::cin >> n;

std::vector<int> arr(n);
for (int& num : arr) {
std::cin >> num;
}

plusMinus(arr);

return 0;
}
