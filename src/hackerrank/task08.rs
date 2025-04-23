#include <iostream>
#include <vector>
#include <algorithm> // for max_element

int birthdayCakeCandles(const std::vector<int>& candles) {
int tallest = *std::max_element(candles.begin(), candles.end());
int count = 0;
for (int height : candles) {
if (height == tallest) {
count++;
}
}
return count;
}

int main() {
int n;
std::cin >> n;
std::vector<int> candles(n);
for (int i = 0; i < n; ++i) {
std::cin >> candles[i];
}
std::cout << birthdayCakeCandles(candles) << std::endl;
return 0;
}
