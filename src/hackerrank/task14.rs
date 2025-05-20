#include <iostream>
#include <vector>
using namespace std;

pair<int, int> breakingRecords(const vector<int>& scores) {
int maxRecord = scores[0];
int minRecord = scores[0];
int maxBreaks = 0;
int minBreaks = 0;

for (size_t i = 1; i < scores.size(); ++i) {
if (scores[i] > maxRecord) {
maxRecord = scores[i];
++maxBreaks;
} else if (scores[i] < minRecord) {
minRecord = scores[i];
++minBreaks;
}
}

return {maxBreaks, minBreaks};
}

int main() {
int n;
cin >> n;
vector<int> scores(n);

for (int i = 0; i < n; ++i) {
cin >> scores[i];
}

auto [maxBreaks, minBreaks] = breakingRecords(scores);
cout << maxBreaks << " " << minBreaks << endl;

return 0;
}
