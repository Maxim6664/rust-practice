#include <iostream>
#include <vector>
#include <unordered_map>
using namespace std;

int migratoryBirds(const vector<int>& arr) {
unordered_map<int, int> freq;
for (int bird : arr) {
freq[bird]++;
}

int max_freq = 0;
int result = 0;

for (int type = 1; type <= 5; ++type) {
if (freq[type] > max_freq) {
max_freq = freq[type];
result = type;
} else if (freq[type] == max_freq && type < result) {
result = type;
}
}

return result;
}

int main() {
int n;
cin >> n;
vector<int> arr(n);

for (int i = 0; i < n; ++i) {
cin >> arr[i];
}

cout << migratoryBirds(arr) << endl;
return 0;
}
