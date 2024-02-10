# Define a function to calculate the factorial of a number
function factorial(n)
    if n == 0
        return 1
    else
        return n * factorial(n - 1)
    end
end

# Calculate the factorial of 5 and print the result
result = factorial(5)
println("The factorial of 5 is $result")
