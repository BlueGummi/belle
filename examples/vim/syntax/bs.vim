highlight DirectiveColor guifg=#F08080 gui=bold ctermfg=LightRed cterm=bold
highlight InstructionColor guifg=#87CEEB ctermfg=LightBlue
highlight NumberShade1 guifg=#AFEEEE ctermfg=Cyan
highlight NumberShade2 guifg=#D8BFD8 ctermfg=Magenta
highlight NumberShade3 guifg=#98FB98 ctermfg=LightGreen
highlight bsLabelColor guifg=#DDA0DD gui=bold ctermfg=Magenta cterm=bold
highlight bsLabelDeclColor guifg=#DD30DD gui=bold ctermfg=Magenta cterm=bold
highlight bsVariable guifg=#77DD77 gui=bold ctermfg=Green cterm=bold
highlight bsMemory guifg=#FFD700 ctermfg=Yellow
highlight bsMemoryIndirect guifg=#FFA07A ctermfg=LightRed
highlight bsPound guifg=#A9A9A9 ctermfg=Gray
highlight bsRegister guifg=#FFFFA0 ctermfg=LightYellow
highlight bsRegisterIndirect guifg=#87CEFA ctermfg=LightBlue

highlight bsString guifg=#D3D3D3 ctermfg=LightGrey cterm=italic
highlight bsSingleChar guifg=#F08080 ctermfg=LightRed cterm=italic

syntax match bsComment ";.*$"
highlight link bsComment Comment

syntax match bsDirective /^\s*\.\(word\|asciiz\)\s*/
highlight link bsDirective DirectiveColor

syntax match bsHex /\<0x[0-9A-Fa-f]\+\>/
highlight link bsHex NumberShade1

syntax match bsBinary /\<0b[01]\+\>/
highlight link bsBinary NumberShade2

syntax match bsDecimal /\<[0-9]\+\>/
highlight link bsDecimal NumberShade3

syntax match bsSingleChar /'\(.\)'/
highlight link bsSingleChar bsSingleChar

syntax match bsString /"\(.\+\)"/
highlight link bsString bsString

syntax match bsRegister /\<r[0-7]\>/
highlight link bsRegister bsRegister

syntax match bsRegisterIndirect /&r[0-7]\>/
highlight link bsRegisterIndirect bsRegisterIndirect

syntax match bsLabel /@\w\+/
highlight link bsLabel bsLabelColor

syntax match bsLabelDecl /\w\+:\s*/
highlight link bsLabelDecl bsLabelDeclColor

syntax match bsVariable /\w\+\s*=\s*\ze[0-9]\+/
highlight link bsVariable bsVariable

syntax match bsMemory /[\[\]]/
highlight link bsMemory bsMemory

syntax match bsMemoryIndirect /&\[\]/
highlight link bsMemoryIndirect bsMemoryIndirect

syntax match bsPound /#\ze\(0x[0-9A-Fa-f]\+\|0b[01]\+\|[0-9]\+\)/
highlight link bsPound bsPound

syntax match bsMemoryDollar /\$\ze[0-9A-Fa-f]\+/
highlight link bsMemoryDollar bsMemory

syntax keyword bsInstruction hlt add bo bno pop div ret ld st jmp bz cmp mul push int mov lea bl bg bnz j be bne
highlight link bsInstruction InstructionColor

highlight CpuDirectiveColor guifg=#B22222 gui=bold ctermfg=DarkRed cterm=bold

syntax match bsCpuDirective /^\s*\.\(start\|ssp\|sbp\)\s*/
highlight link bsCpuDirective CpuDirectiveColor
