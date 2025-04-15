#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

void miniMaxSum(vector<long long> arr) {
sort(arr.begin(), arr.end());

long long minSum = 0;
long long maxSum = 0;

for (int i = 0; i < 4; i++) {
minSum += arr[i];
}

for (int i = 1; i < 5; i++) {
maxSum += arr[i];
}

cout << minSum << " " << maxSum << endl;
}

int main() {
vector<long long> arr(5);
for (int i = 0; i < 5; i++) {
cin >> arr[i];
}

miniMaxSum(arr);

return 0;
}
