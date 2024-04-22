#include <stdio.h>
int main (void) {
float nums;
float sum;
float a;
float b;
float c;
printf("How many fibonacci numbers do you want?\n");
if (0 == scanf("%f", &nums)) {
nums = 0;
scanf("%*s");
}
printf("\n");
sum = 0;
a = 0;
b = 1;
while(nums>0) {
printf("%.2f\n", (float)(a));
sum = sum+a;
c = a+b;
a = b;
b = c;
nums = nums-1;
}
printf("%.2f\n", (float)(sum));
return 0;
}
