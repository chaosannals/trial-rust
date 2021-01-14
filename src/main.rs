use std::io::Result;

mod hosts;
mod ngx;

fn main() -> Result<()> {
    let vhosts = ngx::get_vhosts();
    hosts::set_hosts(&vhosts)
}
