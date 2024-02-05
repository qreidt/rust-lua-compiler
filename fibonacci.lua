-- Function to generate the Fibonacci sequence up to n terms
function generateFibonacci(n)
    local fibonacciSequence = {0, 1}

    for i = 3, n do
        local nextTerm = fibonacciSequence[i - 1] + fibonacciSequence[i - 2]
        table.insert(fibonacciSequence, nextTerm)
    end

    return fibonacciSequence
end

-- Function to print the Fibonacci sequence
function printFibonacci(sequence)
    for i, term in ipairs(sequence) do
        print("Fibonacci[" .. i .. "] = " .. term)
    end
end

-- Specify the number of terms in the Fibonacci sequence
local numberOfTerms = 10

-- Generate and print the Fibonacci sequence
local fibonacciSequence = generateFibonacci(numberOfTerms)
printFibonacci(fibonacciSequence)
