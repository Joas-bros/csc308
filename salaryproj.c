#include <stdio.h>

int main()
{
    int age, experience;
    int salary;

    // Taking input for experience (1 for experienced, 0 for inexperienced)
    printf("Enter your experience (1 for experienced, 0 for inexperienced): ");
    scanf("%d", &experience);

    // Taking input for age
    printf("Enter your age: ");
    scanf("%d", &age);

    // Salary determination based on experience and age
    if (experience == 1)
    { // Experienced person
        if (age >= 40)
        {
            salary = 560000;
        }
        else if (age >= 30)
        {
            salary = 480000;
        }
        else if (age < 28)
        {
            salary = 300000;
        }
        else
        {
            salary = 100000;
        }
    }
    else
    { // Inexperienced person
        salary = 100000;
    }

    // Displaying the salary
    printf("Your salary is: N%d\n", salary);

    return 0;
}
