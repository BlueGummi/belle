syntax clear

syntax match bsComment ";.*$"
highlight def link bsComment Comment

syntax keyword bsDirective .start .ssp .sbp .asciiz .word
highlight def link bsDirective Statement

syntax match bsHex /\<0x[0-9A-Fa-f]\+/
highlight def link bsHex Number

syntax match bsBinary /\<0b[01]\+/
highlight def link bsBinary Number

syntax match bsDecimal /\<[0-9]\+/
highlight def link bsDecimal Number

syntax match bsSingleChar /'\(.\)'/
highlight def link bsSingleChar String

syntax match bsString /"\(.\+\)"/
highlight def link bsString String

syntax match bsRegister /\<r[0-7]\>/
highlight def link bsRegister Identifier

syntax match bsLabel /@\w\+/
highlight def link bsLabel Identifier

syntax match bsLabelDecl /\w\+:\s*/
highlight def link bsLabelDecl Identifier

syntax match bsVariable /\w\+\s*=\s*[0-9]\+/
highlight def link bsVariable Identifier
