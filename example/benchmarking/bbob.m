function scores = zakharovfcn(x)
    n = size(x, 2);
    comp1 = 0;
    comp2 = 0;
    
    for i = 1:n
        comp1 = comp1 + (x(:, i) .^ 2);
        comp2 = comp2 + (0.5 * i * x(:, i));
    end
     
    scores = comp1 + (comp2 .^ 2) + (comp2 .^ 4);
end

function scores = xinsheyangn4fcn(x)
     scores = (sum(sin(x) .^2, 2) - exp(-sum(x .^ 2, 2))) .* exp(-sum(sin(sqrt(abs(x))) .^2, 2));
end

function scores = xinsheyangn3fcn(x, beta, m)
   if nargin < 2
       beta = 15;
   end
   if nargin < 3
       m = 5;
   end
   
   scores = exp(-sum((x / beta).^(2*m), 2)) - (2 * exp(-sum(x .^ 2, 2)) .* prod(cos(x) .^ 2, 2));
end

function scores = shubert3fcn(x)
    n = size(x, 2);
    
    scores = 0;
    for i = 1:n
        for j = 1:5
            scores = scores + j * sin(((j + 1) * x(:, i)) + j);
        end
    end
end

function scores = schwefel223fcn(x)
    scores = sum(x .^10, 2);
end

function scores = schwefel222fcn(x)

    absx = abs(x);
    scores = sum(absx, 2) + prod(absx, 2);
end

function scores = schwefel221fcn(x)
    scores = max(abs(x), [], 2);
end

function scores = schwefel220fcn(x)
    scores = sum(abs(x), 2);
end

function scores = salomonfcn(x)
    x2 = x .^ 2;
    sumx2 = sum(x2, 2);
    sqrtsx2 = sqrt(sumx2);
    
    scores = 1 - cos(2 .* pi .* sqrtsx2) + (0.1 * sqrtsx2);
end

function scores = rosenbrockfcn(x)
    scores = 0;
    n = size(x, 2);
    assert(n >= 1, 'Given input X cannot be empty');
    a = 1;
    b = 100;
    for i = 1 : (n-1)
        scores = scores + (b * ((x(:, i+1) - (x(:, i).^2)) .^ 2)) + ((a - x(:, i)) .^ 2);
    end
end

function scores = sumsquaresfcn(x)
   
   [m, n] = size(x);
   x2 = x .^2;
   I = repmat(1:n, m, 1);
   scores = sum( I .* x2, 2);
   
end

function scores = styblinskitankfcn(x)
    n = size(x, 2);
    scores = 0;
    for i = 1:n
        scores = scores + ((x(:, i) .^4) - (16 * x(:, i) .^ 2) + (5 * x(:, i)));
    end
    scores = 0.5 * scores;
end

function scores = shubertfcn(x)
    n = size(x, 2);
    
    scores = 1;
    for i = 1:n
        inner_sum = 0;
        for j = 1:5
            inner_sum = inner_sum + j * cos(((j + 1) * x(:, i)) + j);
        end
        scores = inner_sum .* scores;
    end
end

function scores = shubert4fcn(x)
    n = size(x, 2);
    
    scores = 0;
    for i = 1:n
        for j = 1:5
            scores = scores + j * cos(((j + 1) * x(:, i)) + j);
        end
    end
end

