#include <iostream>
#include <vector>
#include <unordered_map>
using namespace std;

int sockMerchant(int n, const vector<int>& ar) {
unordered_map<int, int> color_count;

// Підрахунок кількості кожного кольору
for (int color : ar) {
color_count[color]++;
}

int pairs = 0;

// Для кожного кольору — кількість пар = count / 2
for (const auto& entry : color_count) {
pairs += entry.second / 2;
}

return pairs;
}

int main() {
int n;
cin >> n;

vector<int> ar(n);
for (int i = 0; i < n; ++i) {
cin >> ar[i];
}

cout << sockMerchant(n, ar) << endl;
return 0;
}
