/*Rust通过修改注册表启用或禁用任务管理器
Published: 2018-03-21
By Yieldone
tags: Rust

开发全屏应用的时候，除了需要禁用一些ALT+F4，Win+Tab，Alt+Tab外，任务管理器也应该禁用，了解一番后，发现Ctrl+ALT+DEL组合键是Ring0级别，很难屏蔽，不能通过简单的HOOK方式让其失效，于是研究了一个最简单的方法，通过修改注册表启用禁用任务管理器

路径：HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Policies\\System

这个注册表路径里的DisableTaskmgr字段，REG_DWORD类型，如果其值为1，则禁用任务管理器，为0则启动任务管理器（修改后即时生效）

这篇文章是C语言的例子：Windows API 教程（十） 注册表操作

Rust有个名为winreg的crate，可以方便的操作注册表
*/

extern crate winreg;
use std::path::Path;
use winreg::RegKey;
use winreg::enums::*;

fn main() {

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software\\Microsoft\\Windows\\CurrentVersion\\Policies").join("System");
    let key = hkcu.create_subkey(&path).unwrap();

    key.set_value("DisableTaskmgr", &1u32).unwrap();
    let sz_val: String = key.get_value("DisableTaskmgr").unwrap();
    println!("DisableTaskmgr = {}", sz_val);

}
// 上边的代码运行会禁用任务管理器，将&1u32修改为&0u32是启动任务管理器