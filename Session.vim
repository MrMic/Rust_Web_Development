let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/DEV/RUST_WEB_DEVELOPMENT/CH_10/rust-web-dev-config
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +102 ~/.config/AstroNvim/lua/user/mappings.lua
badd +10 ~/.config/AstroNvim/lua/user/lsp/config/phpactor.lua
badd +142 ~/.config/AstroNvim/lua/user/plugins/community.lua
badd +1 ~/.config/AstroNvim/lua/user/plugins/user.lua
badd +43 ~/.config/AstroNvim/lua/user/init.lua
badd +11 ~/.config/AstroNvim/lua/user/lsp/config/rust_analyzer.lua
argglobal
%argdel
edit ~/.config/AstroNvim/lua/user/mappings.lua
argglobal
balt ~/.config/AstroNvim/lua/user/plugins/user.lua
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=99
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
13,16fold
12,18fold
76,78fold
80,82fold
84,86fold
88,90fold
98,100fold
8,102fold
106,108fold
104,109fold
111,113fold
6,115fold
let &fdl = &fdl
6
normal! zo
8
normal! zo
let s:l = 102 - ((32 * winheight(0) + 25) / 51)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 102
normal! 041|
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
