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
badd +14 src/main.rs
badd +94 src/profanity.rs
badd +113 src/routes/authentication.rs
argglobal
%argdel
edit src/routes/authentication.rs
argglobal
balt src/profanity.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=99
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
1,5fold
7,9fold
16,19fold
22,25fold
13,26fold
29,33fold
42,43fold
42,44fold
41,44fold
45,47fold
40,48fold
49,50fold
38,52fold
37,55fold
36,56fold
59,61fold
69,71fold
64,80fold
85,90fold
83,94fold
99,101fold
98,104fold
98,105fold
97,106fold
108,110fold
116,127fold
112,128fold
let &fdl = &fdl
let s:l = 113 - ((25 * winheight(0) + 25) / 51)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 113
normal! 023|
if exists(':tcd') == 2 | tcd ~/DEV/RUST_WEB_DEVELOPMENT/CH_10/rust-web-dev-config | endif
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=10
let &shortmess = s:shortmess_save
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
