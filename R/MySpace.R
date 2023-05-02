#' @export
.DollarNames.MySpace = function(env, pattern = "") {
  ls(MySpace,pattern=pattern)
}

#' @export
print.MySpace = function(x,...) {
  old_x = x
  if(x$length()>10) {
    x = x$subset(1:10)
    elipsis = "... "
  } else {
    elipsis = ""
  }
  print(paste("myspace vector ",x$data_id()," [",paste(x$to_text(),collapse =", "),elipsis,"]"))
  invisible(old_x)
}

#' @export
length.MySpace = function(x,... ) {
  x$length()
}

#' @export
"[.MySpace" = function(self, idx, ...) {
  x$subset(idx)
}

#' @export
myspace_vec = function(x = integer()) {
  helloextendr:::MySpace$new(x)
}

#' @export
as.data.frame.MySpace = function (
    x, row.names = NULL, optional = FALSE, ..., nm = deparse1(substitute(x))
) {
    force(nm)
    nrows <- length(x)
    if (
      !(is.null(row.names) || (is.character(row.names)&&length(row.names) == nrows))
    ) {
        warning(gettextf("'row.names' is not a character vector of length %d -- omitting it. Will be an error!", 
            nrows), domain = NA)
        row.names <- NULL
    }
    if (is.null(row.names)) {
        if (nrows == 0L) 
            row.names <- character()
        else if (length(row.names <- names(x)) != nrows || anyDuplicated(row.names)) 
            row.names <- .set_row_names(nrows)
    }
    if (!is.null(names(x))) names(x) <- NULL
    value <- list(x)
    if (!optional)  names(value) <- nm
    structure(value, row.names = row.names, class = "data.frame")
}

#' @export
format.MySpace = function(x, ...) {
  xx <- x$to_text()
  names(xx) = names(x)
  xx
}
