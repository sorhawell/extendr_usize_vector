
library(reprex)



reprex::reprex({
  library(helloextendr)
  
  #make
  x = usize_vec(c(1:5,NA_integer_))
  
  #index it
  x[1:3]
  
  length(x)
  
  #put in a data.frame
  df = data.frame(
    usize_vec = x,
    i32_vec = seq_along(x) #seq_along just needs length to work
  )
  
  #subset combined vector
  df[1:3,]
})



