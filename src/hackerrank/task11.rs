#include <iostream>
#include <vector>

using namespace std;

int roundGrade(int grade) {
if (grade < 38) return grade;

int nextMultipleOf5 = ((grade / 5) + 1) * 5;
if (nextMultipleOf5 - grade < 3)
return nextMultipleOf5;
else
return grade;
}

int main() {
int n;
cin >> n;
vector<int> grades(n);

for (int i = 0; i < n; ++i) {
cin >> grades[i];
}

for (int i = 0; i < n; ++i) {
cout << roundGrade(grades[i]) << endl;
}

return 0;
}
