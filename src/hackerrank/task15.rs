#include <iostream>
#include <vector>
using namespace std;

int birthday(vector<int> s, int d, int m) {
int count = 0;

for (int i = 0; i <= s.size() - m; ++i) {
int sum = 0;
for (int j = 0; j < m; ++j) {
sum += s[i + j];
}
if (sum == d) {
count++;
}
}

return count;
}

int main() {
int n;
cin >> n;
vector<int> s(n);

for (int i = 0; i < n; ++i) {
cin >> s[i];
}

int d, m;
cin >> d >> m;

cout << birthday(s, d, m) << endl;
return 0;
}
