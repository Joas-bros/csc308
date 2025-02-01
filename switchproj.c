#include <stdio.h>

int main()
{
    char type;
    printf("Enter the type of input (c for character, i for integer, f for float): ");
    scanf(" %c", &type);

    switch (type)
    {
    case 'c':
    { // Character input
        char ch;
        printf("Enter a character: ");
        scanf(" %c", &ch);
        printf("\nNext four characters in multiples of 3:\n");
        for (int i = 1; i <= 4; i++)
        {
            printf("Character: %c, ASCII: %d\n", ch + (3 * i), ch + (3 * i));
        }
        printf("Size of character: %lu bytes\n", sizeof(ch));
        break;
    }

    case 'i':
    { // Integer input
        int num;
        printf("Enter an integer: ");
        scanf("%d", &num);
        printf("\nNext four integers in multiples of 3:\n");
        for (int i = 1; i <= 4; i++)
        {
            printf("Integer: %d\n", num + (3 * i));
        }
        printf("Size of integer: %lu bytes\n", sizeof(num));
        break;
    }

    case 'f':
    { // Float input
        float num;
        printf("Enter a float: ");
        scanf("%f", &num);
        printf("\nNext four floats in multiples of 3:\n");
        for (int i = 1; i <= 4; i++)
        {
            printf("Float: %.2f\n", num + (3.0 * i));
        }
        printf("Size of float: %lu bytes\n", sizeof(num));
        break;
    }

    default:
        printf("Invalid input type! Please enter 'c', 'i', or 'f'.\n");
    }

    return 0;
}
