set term png

# Plot 1
set output "hist_fold.png"
set title "x0 xor x1 xor x2 xor x3"
set grid
set xtics rotate by -45
set boxwidth 0.5
set style fill solid
set xrange [0:255]
plot "hist_fold.txt" using 1:2 with boxes

# Plot 2
set output "hist_other_fold.png"
set title "(x0 + x2) xor (x1 + x3)"
plot "hist_other_fold.txt" using 1:2 with boxes

# Plot 3
set output "hist_mod_256.png"
set title "x % 256"
plot "hist_mod_256.txt" using 1:2 with boxes
