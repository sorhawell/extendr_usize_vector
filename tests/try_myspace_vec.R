
library(reprex)



reprex::reprex({
  library(helloextendr)
  
  #make
  l = list(
    ms_i32 = myspace_vec(c(1:3 ,NA_integer_)),
    ms_f64 = myspace_vec(c(1.1,2.2,3.3,4.4))
  )
  
  l
  
  
})



