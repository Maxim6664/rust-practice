#include <iostream>
#include <vector>
using namespace std;

vector<int> gradingStudents(const vector<int>& grades) {
vector<int> roundedGrades;
for (int grade : grades) {
if (grade < 38) {
roundedGrades.push_back(grade);
} else {
int nextMultipleOf5 = ((grade / 5) + 1) * 5;
if (nextMultipleOf5 - grade < 3) {
roundedGrades.push_back(nextMultipleOf5);
} else {
roundedGrades.push_back(grade);
}
}
}
return roundedGrades;
}

int main() {
int n;
cin >> n;
vector<int> grades(n);

for (int i = 0; i < n; ++i) {
cin >> grades[i];
}

vector<int> result = gradingStudents(grades);

for (int grade : result) {
cout << grade << endl;
}

return 0;
}
