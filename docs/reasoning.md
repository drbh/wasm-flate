# Mathematical Reasoning and Calculator

We can prove if sending compressed data is worth the incurred computational time by performing a simple calculation.

If the **time to send** a **file** is greater then the **time to compress** plus the **time to send** and the **time to decompress** then we should opt for the second method.  

This can be thought of as the following psudeo equation.
```
time to send > ( time to compress + time to send compressed + time to decompress )
```

How do we know the time to compress and the time to send in relation to a specific file on a specific network?  

Well we can estimate for now - and then run benchmarks to evulate our theory.  

Taking file size and network speed into account we can rewrite the above formula like:  
```
( file size / network speed ) > ( 
                                    ( file size / compress speed ) + 
                                    ( compressed file size / network speed ) + 
                                    ( file size / decompression speed )
                                )
```

We can simpilfy our equation by attesting that we can compress and decompress at the same speed (they're roughly equal). We also have to guess the compressed file size relative to it's original size and add a variable to evaluate that. 

Next we port these psuedo equations into Python code and make for a simple calculator - this will tell us how many seconds we'll save on a specific file sent over a specific network. We will need to input the size of file to send in MBs, the compression ratio to full size, the compression speed in MB/s and the network speed in MB/s *not Mbps*.  

We'll randomly assign the file to 10 MBs, we assume we can compress to 40% of the original size, we expect compression at 32 MB/s and we'll take the global internet speed average of 5.6 Mbps which is 0.7 MB/s ( MB/s == Mbps/8.0 ).  

We use 32 MB/s for the algo speed - but it is actually much much faster - at closer to 90 MBs based on experience and quoted in this [paper](https://cran.r-project.org/web/packages/brotli/vignettes/brotli-2015-09-22.pdf)  

#### Calculator

```python
f   = 10    # size of file to send in MBs
cr  = 0.40  # compression ratio to full size
cds = 32.0  # MB/s of DEFLATE algo
ns  = 0.7   # Network MB/s == (Mbps/8.0)

time_saved = ( f / ns ) - ( ( (cr * f) / ns ) + ( 2 * ( f / cds ) ) )
size_saved = f - ( f * cr )
print( "time saved: %s secs\nsize saved: %s MBs" % ( round( time_saved, 1 ), size_saved ) )

### OUTPUT
## time saved: 7.9 secs
## size saved: 6.0 MBs
```

We can see that even in our exaggerated example where we underestimate how fast and how small we can compress a file we still save signifcant time on 10 MB/s. Remember this is based on the global internet speed average which is much lower then many countries - for instance South Koreas average is 27 Mbps or 3.375 MB/s


```python
f   = 10      # size of file to send in MBs
cr  = 0.40    # compression ratio to full size
cds = 32.0    # MB/s of DEFLATE algo
ns  = 3.375   # Network MB/s == (Mbps/8.0)

time_saved = ( f / ns ) - ( ( (cr * f) / ns ) + ( 2 * ( f / cds ) ) )
size_saved = f - ( f * cr )
print( "time saved: %s secs\nsize saved: %s MBs" % ( round( time_saved, 1 ), size_saved ) )

### OUTPUT
## time saved: 1.2 secs
## size saved: 6.0 MBs
```
Even with those inputs we save 1.2 seconds and 6 MBs of data! 🙌

In short it is likely UX enchancing to compress data and decompress versus sending large files. In the following benchmarks we'll see why using `wasm-flate` is the fastest and most efficent way to compress on both the server and client sides!  
