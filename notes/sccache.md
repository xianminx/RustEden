# Sccache
Sccache is created by Mozilla to accelerate Rust code compiling. 


* install: `brew install scache`
* config: `cho "export RUSTC_WRAPPER=`which sccache`" >> ~/.profile; source ~/.profile`
* check: `sccache --show-stats`