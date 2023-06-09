# Generated by extendr: Do not edit by hand

# nolint start

#
# This file was created with the following call:
#   .Call("wrap__make_helloextendr_wrappers", use_symbols = TRUE, package_name = "helloextendr")

#' @docType package
#' @usage NULL
#' @useDynLib helloextendr, .registration = TRUE
NULL

#' Return string `"Hello world!"` to R.
#' @export
hello_world <- function() .Call(wrap__hello_world)

MySpace <- new.env(parent = emptyenv())

MySpace$new <- function(robj) .Call(wrap__MySpace__new, robj)

MySpace$subset <- function(idx) .Call(wrap__MySpace__subset, self, idx)

MySpace$length <- function() .Call(wrap__MySpace__length, self)

MySpace$to_text <- function() .Call(wrap__MySpace__to_text, self)

MySpace$data_id <- function() .Call(wrap__MySpace__data_id, self)

#' @export
`$.MySpace` <- function (self, name) { func <- MySpace[[name]]; environment(func) <- environment(); func }

#' @export
`[[.MySpace` <- `$.MySpace`

VecUsize <- new.env(parent = emptyenv())

VecUsize$new <- function(robj) .Call(wrap__VecUsize__new, robj)

VecUsize$subset <- function(idx) .Call(wrap__VecUsize__subset, self, idx)

VecUsize$length <- function() .Call(wrap__VecUsize__length, self)

VecUsize$to_text <- function() .Call(wrap__VecUsize__to_text, self)

#' @export
`$.VecUsize` <- function (self, name) { func <- VecUsize[[name]]; environment(func) <- environment(); func }

#' @export
`[[.VecUsize` <- `$.VecUsize`


# nolint end
