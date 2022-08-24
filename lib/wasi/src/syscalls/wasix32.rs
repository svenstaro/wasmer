#![deny(dead_code)]
use crate::{WasiEnv, WasiError, WasiState, WasiThread};
use wasmer::{FunctionEnvMut, Memory, Memory32, MemorySize, StoreMut, WasmPtr, WasmSlice};
use wasmer_wasi_types::*;
use wasmer_wasi_types_generated::wasi::{
    Advice, BusErrno, Clockid, Dircookie, Errno, Event, Fd, Fdflags, Fdstat, Rights, Sockoption,
    Sockstatus, Socktype, Streamsecurity, Subscription, Timestamp,
};

type MemoryType = Memory32;
type MemoryOffset = u32;

pub(crate) fn args_get(
    ctx: FunctionEnvMut<WasiEnv>,
    argv: WasmPtr<WasmPtr<u8, MemoryType>, MemoryType>,
    argv_buf: WasmPtr<u8, MemoryType>,
) -> Errno {
    super::args_get::<MemoryType>(ctx, argv, argv_buf)
}

pub(crate) fn args_sizes_get(
    ctx: FunctionEnvMut<WasiEnv>,
    argc: WasmPtr<MemoryOffset, MemoryType>,
    argv_buf_size: WasmPtr<MemoryOffset, MemoryType>,
) -> Errno {
    super::args_sizes_get::<MemoryType>(ctx, argc, argv_buf_size)
}

pub(crate) fn clock_res_get(
    ctx: FunctionEnvMut<WasiEnv>,
    clock_id: Clockid,
    resolution: WasmPtr<Timestamp, MemoryType>,
) -> Errno {
    super::clock_res_get::<MemoryType>(ctx, clock_id, resolution)
}

pub(crate) fn clock_time_get(
    ctx: FunctionEnvMut<WasiEnv>,
    clock_id: Clockid,
    precision: Timestamp,
    time: WasmPtr<Timestamp, MemoryType>,
) -> Errno {
    super::clock_time_get::<MemoryType>(ctx, clock_id, precision, time)
}

pub(crate) fn environ_get(
    ctx: FunctionEnvMut<WasiEnv>,
    environ: WasmPtr<WasmPtr<u8, MemoryType>, MemoryType>,
    environ_buf: WasmPtr<u8, MemoryType>,
) -> Errno {
    super::environ_get::<MemoryType>(ctx, environ, environ_buf)
}

pub(crate) fn environ_sizes_get(
    ctx: FunctionEnvMut<WasiEnv>,
    environ_count: WasmPtr<MemoryOffset, MemoryType>,
    environ_buf_size: WasmPtr<MemoryOffset, MemoryType>,
) -> Errno {
    super::environ_sizes_get::<MemoryType>(ctx, environ_count, environ_buf_size)
}

pub(crate) fn fd_advise(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    offset: __wasi_filesize_t,
    len: __wasi_filesize_t,
    advice: Advice,
) -> Errno {
    super::fd_advise(ctx, fd, offset, len, advice)
}

pub(crate) fn fd_allocate(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    offset: __wasi_filesize_t,
    len: __wasi_filesize_t,
) -> Errno {
    super::fd_allocate(ctx, fd, offset, len)
}

pub(crate) fn fd_close(ctx: FunctionEnvMut<WasiEnv>, fd: Fd) -> Errno {
    super::fd_close(ctx, fd)
}

pub(crate) fn fd_datasync(ctx: FunctionEnvMut<WasiEnv>, fd: Fd) -> Errno {
    super::fd_datasync(ctx, fd)
}

pub(crate) fn fd_fdstat_get(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    buf_ptr: WasmPtr<Fdstat, MemoryType>,
) -> Errno {
    super::fd_fdstat_get::<MemoryType>(ctx, fd, buf_ptr)
}

pub(crate) fn fd_fdstat_set_flags(ctx: FunctionEnvMut<WasiEnv>, fd: Fd, flags: Fdflags) -> Errno {
    super::fd_fdstat_set_flags(ctx, fd, flags)
}

pub(crate) fn fd_fdstat_set_rights(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    fs_rights_base: Rights,
    fs_rights_inheriting: Rights,
) -> Errno {
    super::fd_fdstat_set_rights(ctx, fd, fs_rights_base, fs_rights_inheriting)
}

pub(crate) fn fd_filestat_get(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    buf: WasmPtr<__wasi_filestat_t, MemoryType>,
) -> Errno {
    super::fd_filestat_get::<MemoryType>(ctx, fd, buf)
}

pub(crate) fn fd_filestat_set_size(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    st_size: __wasi_filesize_t,
) -> Errno {
    super::fd_filestat_set_size(ctx, fd, st_size)
}

pub(crate) fn fd_filestat_set_times(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    st_atim: Timestamp,
    st_mtim: Timestamp,
    fst_flags: __wasi_fstflags_t,
) -> Errno {
    super::fd_filestat_set_times(ctx, fd, st_atim, st_mtim, fst_flags)
}

pub(crate) fn fd_pread(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    iovs: WasmPtr<__wasi_iovec_t<MemoryType>, MemoryType>,
    iovs_len: MemoryOffset,
    offset: __wasi_filesize_t,
    nread: WasmPtr<MemoryOffset, MemoryType>,
) -> Result<Errno, WasiError> {
    super::fd_pread::<MemoryType>(ctx, fd, iovs, iovs_len, offset, nread)
}

pub(crate) fn fd_prestat_get(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    buf: WasmPtr<__wasi_prestat_t, MemoryType>,
) -> Errno {
    super::fd_prestat_get::<MemoryType>(ctx, fd, buf)
}

pub(crate) fn fd_prestat_dir_name(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
) -> Errno {
    super::fd_prestat_dir_name::<MemoryType>(ctx, fd, path, path_len)
}

pub(crate) fn fd_pwrite(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    iovs: WasmPtr<__wasi_ciovec_t<MemoryType>, MemoryType>,
    iovs_len: MemoryOffset,
    offset: __wasi_filesize_t,
    nwritten: WasmPtr<MemoryOffset, MemoryType>,
) -> Result<Errno, WasiError> {
    super::fd_pwrite::<MemoryType>(ctx, fd, iovs, iovs_len, offset, nwritten)
}

pub(crate) fn fd_read(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    iovs: WasmPtr<__wasi_iovec_t<MemoryType>, MemoryType>,
    iovs_len: MemoryOffset,
    nread: WasmPtr<MemoryOffset, MemoryType>,
) -> Result<Errno, WasiError> {
    super::fd_read::<MemoryType>(ctx, fd, iovs, iovs_len, nread)
}

pub(crate) fn fd_readdir(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    buf: WasmPtr<u8, MemoryType>,
    buf_len: MemoryOffset,
    cookie: Dircookie,
    bufused: WasmPtr<MemoryOffset, MemoryType>,
) -> Errno {
    super::fd_readdir::<MemoryType>(ctx, fd, buf, buf_len, cookie, bufused)
}

pub(crate) fn fd_renumber(ctx: FunctionEnvMut<WasiEnv>, from: Fd, to: Fd) -> Errno {
    super::fd_renumber(ctx, from, to)
}

pub(crate) fn fd_seek(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    offset: __wasi_filedelta_t,
    whence: __wasi_whence_t,
    newoffset: WasmPtr<__wasi_filesize_t, MemoryType>,
) -> Result<Errno, WasiError> {
    super::fd_seek::<MemoryType>(ctx, fd, offset, whence, newoffset)
}

pub(crate) fn fd_sync(ctx: FunctionEnvMut<WasiEnv>, fd: Fd) -> Errno {
    super::fd_sync(ctx, fd)
}

pub(crate) fn fd_tell(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    offset: WasmPtr<__wasi_filesize_t, MemoryType>,
) -> Errno {
    super::fd_tell::<MemoryType>(ctx, fd, offset)
}

pub(crate) fn fd_write(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    iovs: WasmPtr<__wasi_ciovec_t<MemoryType>, MemoryType>,
    iovs_len: MemoryOffset,
    nwritten: WasmPtr<MemoryOffset, MemoryType>,
) -> Result<Errno, WasiError> {
    super::fd_write::<MemoryType>(ctx, fd, iovs, iovs_len, nwritten)
}

pub(crate) fn path_create_directory(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
) -> Errno {
    super::path_create_directory::<MemoryType>(ctx, fd, path, path_len)
}

pub(crate) fn path_filestat_get(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    flags: __wasi_lookupflags_t,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
    buf: WasmPtr<__wasi_filestat_t, MemoryType>,
) -> Errno {
    super::path_filestat_get::<MemoryType>(ctx, fd, flags, path, path_len, buf)
}

pub(crate) fn path_filestat_set_times(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    flags: __wasi_lookupflags_t,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
    st_atim: Timestamp,
    st_mtim: Timestamp,
    fst_flags: __wasi_fstflags_t,
) -> Errno {
    super::path_filestat_set_times::<MemoryType>(
        ctx, fd, flags, path, path_len, st_atim, st_mtim, fst_flags,
    )
}

pub(crate) fn path_link(
    ctx: FunctionEnvMut<WasiEnv>,
    old_fd: Fd,
    old_flags: __wasi_lookupflags_t,
    old_path: WasmPtr<u8, MemoryType>,
    old_path_len: MemoryOffset,
    new_fd: Fd,
    new_path: WasmPtr<u8, MemoryType>,
    new_path_len: MemoryOffset,
) -> Errno {
    super::path_link::<MemoryType>(
        ctx,
        old_fd,
        old_flags,
        old_path,
        old_path_len,
        new_fd,
        new_path,
        new_path_len,
    )
}

pub(crate) fn path_open(
    ctx: FunctionEnvMut<WasiEnv>,
    dirfd: Fd,
    dirflags: __wasi_lookupflags_t,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
    o_flags: __wasi_oflags_t,
    fs_rights_base: Rights,
    fs_rights_inheriting: Rights,
    fs_flags: Fdflags,
    fd: WasmPtr<Fd, MemoryType>,
) -> Errno {
    super::path_open::<MemoryType>(
        ctx,
        dirfd,
        dirflags,
        path,
        path_len,
        o_flags,
        fs_rights_base,
        fs_rights_inheriting,
        fs_flags,
        fd,
    )
}

pub(crate) fn path_readlink(
    ctx: FunctionEnvMut<WasiEnv>,
    dir_fd: Fd,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
    buf: WasmPtr<u8, MemoryType>,
    buf_len: MemoryOffset,
    buf_used: WasmPtr<MemoryOffset, MemoryType>,
) -> Errno {
    super::path_readlink::<MemoryType>(ctx, dir_fd, path, path_len, buf, buf_len, buf_used)
}

pub(crate) fn path_remove_directory(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
) -> Errno {
    super::path_remove_directory::<MemoryType>(ctx, fd, path, path_len)
}

pub(crate) fn path_rename(
    ctx: FunctionEnvMut<WasiEnv>,
    old_fd: Fd,
    old_path: WasmPtr<u8, MemoryType>,
    old_path_len: MemoryOffset,
    new_fd: Fd,
    new_path: WasmPtr<u8, MemoryType>,
    new_path_len: MemoryOffset,
) -> Errno {
    super::path_rename::<MemoryType>(
        ctx,
        old_fd,
        old_path,
        old_path_len,
        new_fd,
        new_path,
        new_path_len,
    )
}

pub(crate) fn path_symlink(
    ctx: FunctionEnvMut<WasiEnv>,
    old_path: WasmPtr<u8, MemoryType>,
    old_path_len: MemoryOffset,
    fd: Fd,
    new_path: WasmPtr<u8, MemoryType>,
    new_path_len: MemoryOffset,
) -> Errno {
    super::path_symlink::<MemoryType>(ctx, old_path, old_path_len, fd, new_path, new_path_len)
}

pub(crate) fn path_unlink_file(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
) -> Errno {
    super::path_unlink_file::<MemoryType>(ctx, fd, path, path_len)
}

pub(crate) fn poll_oneoff(
    ctx: FunctionEnvMut<WasiEnv>,
    in_: WasmPtr<Subscription, MemoryType>,
    out_: WasmPtr<Event, MemoryType>,
    nsubscriptions: MemoryOffset,
    nevents: WasmPtr<MemoryOffset, MemoryType>,
) -> Result<Errno, WasiError> {
    super::poll_oneoff::<MemoryType>(ctx, in_, out_, nsubscriptions, nevents)
}

pub(crate) fn proc_exit(
    ctx: FunctionEnvMut<WasiEnv>,
    code: __wasi_exitcode_t,
) -> Result<(), WasiError> {
    super::proc_exit(ctx, code)
}

pub(crate) fn proc_raise(ctx: FunctionEnvMut<WasiEnv>, sig: __wasi_signal_t) -> Errno {
    super::proc_raise(ctx, sig)
}

pub(crate) fn random_get(
    ctx: FunctionEnvMut<WasiEnv>,
    buf: WasmPtr<u8, MemoryType>,
    buf_len: MemoryOffset,
) -> Errno {
    super::random_get::<MemoryType>(ctx, buf, buf_len)
}

pub(crate) fn fd_dup(
    ctx: FunctionEnvMut<WasiEnv>,
    fd: Fd,
    ret_fd: WasmPtr<Fd, MemoryType>,
) -> Errno {
    super::fd_dup::<MemoryType>(ctx, fd, ret_fd)
}

pub(crate) fn fd_event(
    ctx: FunctionEnvMut<WasiEnv>,
    initial_val: u64,
    flags: __wasi_eventfdflags,
    ret_fd: WasmPtr<Fd, MemoryType>,
) -> Errno {
    super::fd_event(ctx, initial_val, flags, ret_fd)
}

pub(crate) fn fd_pipe(
    ctx: FunctionEnvMut<WasiEnv>,
    ro_fd1: WasmPtr<Fd, MemoryType>,
    ro_fd2: WasmPtr<Fd, MemoryType>,
) -> Errno {
    super::fd_pipe::<MemoryType>(ctx, ro_fd1, ro_fd2)
}

pub(crate) fn tty_get(
    ctx: FunctionEnvMut<WasiEnv>,
    tty_state: WasmPtr<__wasi_tty_t, MemoryType>,
) -> Errno {
    super::tty_get::<MemoryType>(ctx, tty_state)
}

pub(crate) fn tty_set(
    ctx: FunctionEnvMut<WasiEnv>,
    tty_state: WasmPtr<__wasi_tty_t, MemoryType>,
) -> Errno {
    super::tty_set::<MemoryType>(ctx, tty_state)
}

pub(crate) fn getcwd(
    ctx: FunctionEnvMut<WasiEnv>,
    path: WasmPtr<u8, MemoryType>,
    path_len: WasmPtr<MemoryOffset, MemoryType>,
) -> Errno {
    super::getcwd::<MemoryType>(ctx, path, path_len)
}

pub(crate) fn chdir(
    ctx: FunctionEnvMut<WasiEnv>,
    path: WasmPtr<u8, MemoryType>,
    path_len: MemoryOffset,
) -> Errno {
    super::chdir::<MemoryType>(ctx, path, path_len)
}

pub(crate) fn thread_spawn(
    ctx: FunctionEnvMut<WasiEnv>,
    method: WasmPtr<u8, MemoryType>,
    method_len: MemoryOffset,
    user_data: u64,
    reactor: __wasi_bool_t,
    ret_tid: WasmPtr<__wasi_tid_t, MemoryType>,
) -> Errno {
    super::thread_spawn::<MemoryType>(ctx, method, method_len, user_data, reactor, ret_tid)
}

pub(crate) fn thread_sleep(
    ctx: FunctionEnvMut<WasiEnv>,
    duration: Timestamp,
) -> Result<Errno, WasiError> {
    super::thread_sleep(ctx, duration)
}

pub(crate) fn thread_id(
    ctx: FunctionEnvMut<WasiEnv>,
    ret_tid: WasmPtr<__wasi_tid_t, MemoryType>,
) -> Errno {
    super::thread_id::<MemoryType>(ctx, ret_tid)
}

pub(crate) fn thread_join(
    ctx: FunctionEnvMut<WasiEnv>,
    tid: __wasi_tid_t,
) -> Result<Errno, WasiError> {
    super::thread_join(ctx, tid)
}

pub(crate) fn thread_parallelism(
    ctx: FunctionEnvMut<WasiEnv>,
    ret_parallelism: WasmPtr<MemoryOffset, MemoryType>,
) -> Errno {
    super::thread_parallelism::<MemoryType>(ctx, ret_parallelism)
}

pub(crate) fn thread_exit(
    ctx: FunctionEnvMut<WasiEnv>,
    exitcode: __wasi_exitcode_t,
) -> Result<Errno, WasiError> {
    super::thread_exit(ctx, exitcode)
}

pub(crate) fn sched_yield(ctx: FunctionEnvMut<WasiEnv>) -> Result<Errno, WasiError> {
    super::sched_yield(ctx)
}

pub(crate) fn getpid(
    ctx: FunctionEnvMut<WasiEnv>,
    ret_pid: WasmPtr<__wasi_pid_t, MemoryType>,
) -> Errno {
    super::getpid::<MemoryType>(ctx, ret_pid)
}

pub(crate) fn process_spawn(
    ctx: FunctionEnvMut<WasiEnv>,
    name: WasmPtr<u8, MemoryType>,
    name_len: MemoryOffset,
    chroot: __wasi_bool_t,
    args: WasmPtr<u8, MemoryType>,
    args_len: MemoryOffset,
    preopen: WasmPtr<u8, MemoryType>,
    preopen_len: MemoryOffset,
    stdin: __wasi_stdiomode_t,
    stdout: __wasi_stdiomode_t,
    stderr: __wasi_stdiomode_t,
    working_dir: WasmPtr<u8, MemoryType>,
    working_dir_len: MemoryOffset,
    ret_handles: WasmPtr<__wasi_bus_handles_t, MemoryType>,
) -> BusErrno {
    super::process_spawn::<MemoryType>(
        ctx,
        name,
        name_len,
        chroot,
        args,
        args_len,
        preopen,
        preopen_len,
        stdin,
        stdout,
        stderr,
        working_dir,
        working_dir_len,
        ret_handles,
    )
}

pub(crate) fn bus_open_local(
    ctx: FunctionEnvMut<WasiEnv>,
    name: WasmPtr<u8, MemoryType>,
    name_len: MemoryOffset,
    reuse: __wasi_bool_t,
    ret_bid: WasmPtr<__wasi_bid_t, MemoryType>,
) -> BusErrno {
    super::bus_open_local::<MemoryType>(ctx, name, name_len, reuse, ret_bid)
}

pub(crate) fn bus_open_remote(
    ctx: FunctionEnvMut<WasiEnv>,
    name: WasmPtr<u8, MemoryType>,
    name_len: MemoryOffset,
    reuse: __wasi_bool_t,
    instance: WasmPtr<u8, MemoryType>,
    instance_len: MemoryOffset,
    token: WasmPtr<u8, MemoryType>,
    token_len: MemoryOffset,
    ret_bid: WasmPtr<__wasi_bid_t, MemoryType>,
) -> BusErrno {
    super::bus_open_remote::<MemoryType>(
        ctx,
        name,
        name_len,
        reuse,
        instance,
        instance_len,
        token,
        token_len,
        ret_bid,
    )
}

pub(crate) fn bus_close(ctx: FunctionEnvMut<WasiEnv>, bid: __wasi_bid_t) -> BusErrno {
    super::bus_close(ctx, bid)
}

pub(crate) fn bus_call(
    ctx: FunctionEnvMut<WasiEnv>,
    bid: __wasi_bid_t,
    keep_alive: __wasi_bool_t,
    topic: WasmPtr<u8, MemoryType>,
    topic_len: MemoryOffset,
    format: __wasi_busdataformat_t,
    buf: WasmPtr<u8, MemoryType>,
    buf_len: MemoryOffset,
    ret_cid: WasmPtr<__wasi_cid_t, MemoryType>,
) -> BusErrno {
    super::bus_call::<MemoryType>(
        ctx, bid, keep_alive, topic, topic_len, format, buf, buf_len, ret_cid,
    )
}

pub(crate) fn bus_subcall(
    ctx: FunctionEnvMut<WasiEnv>,
    parent: __wasi_cid_t,
    keep_alive: __wasi_bool_t,
    topic: WasmPtr<u8, MemoryType>,
    topic_len: MemoryOffset,
    format: __wasi_busdataformat_t,
    buf: WasmPtr<u8, MemoryType>,
    buf_len: MemoryOffset,
    ret_cid: WasmPtr<__wasi_cid_t, MemoryType>,
) -> BusErrno {
    super::bus_subcall::<MemoryType>(
        ctx, parent, keep_alive, topic, topic_len, format, buf, buf_len, ret_cid,
    )
}

pub(crate) fn bus_poll(
    ctx: FunctionEnvMut<WasiEnv>,
    timeout: Timestamp,
    events: WasmPtr<u8, MemoryType>,
    nevents: MemoryOffset,
    malloc: WasmPtr<u8, MemoryType>,
    malloc_len: MemoryOffset,
    ret_nevents: WasmPtr<MemoryOffset, MemoryType>,
) -> BusErrno {
    super::bus_poll::<MemoryType>(
        ctx,
        timeout,
        events,
        nevents,
        malloc,
        malloc_len,
        ret_nevents,
    )
}

pub(crate) fn call_reply(
    ctx: FunctionEnvMut<WasiEnv>,
    cid: __wasi_cid_t,
    format: __wasi_busdataformat_t,
    buf: WasmPtr<u8, MemoryType>,
    buf_len: MemoryOffset,
) -> BusErrno {
    super::call_reply::<MemoryType>(ctx, cid, format, buf, buf_len)
}

pub(crate) fn call_fault(
    ctx: FunctionEnvMut<WasiEnv>,
    cid: __wasi_cid_t,
    fault: BusErrno,
) -> BusErrno {
    super::call_fault(ctx, cid, fault)
}

pub(crate) fn call_close(ctx: FunctionEnvMut<WasiEnv>, cid: __wasi_cid_t) -> BusErrno {
    super::call_close(ctx, cid)
}

pub(crate) fn port_bridge(
    ctx: FunctionEnvMut<WasiEnv>,
    network: WasmPtr<u8, MemoryType>,
    network_len: MemoryOffset,
    token: WasmPtr<u8, MemoryType>,
    token_len: MemoryOffset,
    security: Streamsecurity,
) -> Errno {
    super::port_bridge::<MemoryType>(ctx, network, network_len, token, token_len, security)
}

pub(crate) fn port_unbridge(ctx: FunctionEnvMut<WasiEnv>) -> Errno {
    super::port_unbridge(ctx)
}

pub(crate) fn port_dhcp_acquire(ctx: FunctionEnvMut<WasiEnv>) -> Errno {
    super::port_dhcp_acquire(ctx)
}

pub(crate) fn port_addr_add(
    ctx: FunctionEnvMut<WasiEnv>,
    addr: WasmPtr<__wasi_cidr_t, MemoryType>,
) -> Errno {
    super::port_addr_add::<MemoryType>(ctx, addr)
}

pub(crate) fn port_addr_remove(
    ctx: FunctionEnvMut<WasiEnv>,
    addr: WasmPtr<__wasi_addr_t, MemoryType>,
) -> Errno {
    super::port_addr_remove::<MemoryType>(ctx, addr)
}

pub(crate) fn port_addr_clear(ctx: FunctionEnvMut<WasiEnv>) -> Errno {
    super::port_addr_clear(ctx)
}

pub(crate) fn port_addr_list(
    ctx: FunctionEnvMut<WasiEnv>,
    addrs: WasmPtr<__wasi_cidr_t, MemoryType>,
    naddrs: WasmPtr<MemoryOffset, MemoryType>,
) -> Errno {
    super::port_addr_list::<MemoryType>(ctx, addrs, naddrs)
}

pub(crate) fn port_mac(
    ctx: FunctionEnvMut<WasiEnv>,
    ret_mac: WasmPtr<__wasi_hardwareaddress_t, MemoryType>,
) -> Errno {
    super::port_mac::<MemoryType>(ctx, ret_mac)
}

pub(crate) fn port_gateway_set(
    ctx: FunctionEnvMut<WasiEnv>,
    ip: WasmPtr<__wasi_addr_t, MemoryType>,
) -> Errno {
    super::port_gateway_set::<MemoryType>(ctx, ip)
}

pub(crate) fn port_route_add(
    ctx: FunctionEnvMut<WasiEnv>,
    cidr: WasmPtr<__wasi_cidr_t, MemoryType>,
    via_router: WasmPtr<__wasi_addr_t, MemoryType>,
    preferred_until: WasmPtr<__wasi_option_timestamp_t, MemoryType>,
    expires_at: WasmPtr<__wasi_option_timestamp_t, MemoryType>,
) -> Errno {
    super::port_route_add::<MemoryType>(ctx, cidr, via_router, preferred_until, expires_at)
}

pub(crate) fn port_route_remove(
    ctx: FunctionEnvMut<WasiEnv>,
    ip: WasmPtr<__wasi_addr_t, MemoryType>,
) -> Errno {
    super::port_route_remove::<MemoryType>(ctx, ip)
}

pub(crate) fn port_route_clear(ctx: FunctionEnvMut<WasiEnv>) -> Errno {
    super::port_route_clear(ctx)
}

pub(crate) fn port_route_list(
    ctx: FunctionEnvMut<WasiEnv>,
    routes: WasmPtr<__wasi_route_t, MemoryType>,
    nroutes: WasmPtr<MemoryOffset, MemoryType>,
) -> Errno {
    super::port_route_list::<MemoryType>(ctx, routes, nroutes)
}

pub(crate) fn ws_connect(
    ctx: FunctionEnvMut<WasiEnv>,
    url: WasmPtr<u8, MemoryType>,
    url_len: MemoryOffset,
    ret_sock: WasmPtr<Fd, MemoryType>,
) -> Errno {
    super::ws_connect::<MemoryType>(ctx, url, url_len, ret_sock)
}

pub(crate) fn http_request(
    ctx: FunctionEnvMut<WasiEnv>,
    url: WasmPtr<u8, MemoryType>,
    url_len: MemoryOffset,
    method: WasmPtr<u8, MemoryType>,
    method_len: MemoryOffset,
    headers: WasmPtr<u8, MemoryType>,
    headers_len: MemoryOffset,
    gzip: __wasi_bool_t,
    ret_handles: WasmPtr<__wasi_http_handles_t, MemoryType>,
) -> Errno {
    super::http_request::<MemoryType>(
        ctx,
        url,
        url_len,
        method,
        method_len,
        headers,
        headers_len,
        gzip,
        ret_handles,
    )
}

pub(crate) fn http_status(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    status: WasmPtr<__wasi_http_status_t, MemoryType>,
    status_text: WasmPtr<u8, MemoryType>,
    status_text_len: WasmPtr<MemoryOffset, MemoryType>,
    headers: WasmPtr<u8, MemoryType>,
    headers_len: WasmPtr<MemoryOffset, MemoryType>,
) -> Errno {
    super::http_status::<MemoryType>(ctx, sock, status)
}

pub(crate) fn sock_status(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    ret_status: WasmPtr<Sockstatus, MemoryType>,
) -> Errno {
    super::sock_status::<MemoryType>(ctx, sock, ret_status)
}

pub(crate) fn sock_addr_local(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    ret_addr: WasmPtr<__wasi_addr_port_t, MemoryType>,
) -> Errno {
    super::sock_addr_local::<MemoryType>(ctx, sock, ret_addr)
}

pub(crate) fn sock_addr_peer(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    ro_addr: WasmPtr<__wasi_addr_port_t, MemoryType>,
) -> Errno {
    super::sock_addr_peer::<MemoryType>(ctx, sock, ro_addr)
}

pub(crate) fn sock_open(
    ctx: FunctionEnvMut<WasiEnv>,
    af: __wasi_addressfamily_t,
    ty: Socktype,
    pt: __wasi_sockproto_t,
    ro_sock: WasmPtr<Fd, MemoryType>,
) -> Errno {
    super::sock_open::<MemoryType>(ctx, af, ty, pt, ro_sock)
}

pub(crate) fn sock_set_opt_flag(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    opt: Sockoption,
    flag: __wasi_bool_t,
) -> Errno {
    super::sock_set_opt_flag(ctx, sock, opt, flag)
}

pub(crate) fn sock_get_opt_flag(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    opt: Sockoption,
    ret_flag: WasmPtr<__wasi_bool_t, MemoryType>,
) -> Errno {
    super::sock_get_opt_flag::<MemoryType>(ctx, sock, opt, ret_flag)
}

pub fn sock_set_opt_time(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    opt: Sockoption,
    time: WasmPtr<__wasi_option_timestamp_t, MemoryType>,
) -> Errno {
    super::sock_set_opt_time(ctx, sock, opt, time)
}

pub fn sock_get_opt_time(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    opt: Sockoption,
    ret_time: WasmPtr<__wasi_option_timestamp_t, MemoryType>,
) -> Errno {
    super::sock_get_opt_time(ctx, sock, opt, ret_time)
}

pub fn sock_set_opt_size(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    opt: Sockoption,
    size: __wasi_filesize_t,
) -> Errno {
    super::sock_set_opt_size(ctx, sock, opt, size)
}

pub fn sock_get_opt_size(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    opt: Sockoption,
    ret_size: WasmPtr<__wasi_filesize_t, MemoryType>,
) -> Errno {
    super::sock_get_opt_size(ctx, sock, opt, ret_size)
}

pub(crate) fn sock_join_multicast_v4(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    multiaddr: WasmPtr<__wasi_addr_ip4_t, MemoryType>,
    iface: WasmPtr<__wasi_addr_ip4_t, MemoryType>,
) -> Errno {
    super::sock_join_multicast_v4::<MemoryType>(ctx, sock, multiaddr, iface)
}

pub(crate) fn sock_leave_multicast_v4(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    multiaddr: WasmPtr<__wasi_addr_ip4_t, MemoryType>,
    iface: WasmPtr<__wasi_addr_ip4_t, MemoryType>,
) -> Errno {
    super::sock_leave_multicast_v4::<MemoryType>(ctx, sock, multiaddr, iface)
}

pub(crate) fn sock_join_multicast_v6(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    multiaddr: WasmPtr<__wasi_addr_ip6_t, MemoryType>,
    iface: u32,
) -> Errno {
    super::sock_join_multicast_v6::<MemoryType>(ctx, sock, multiaddr, iface)
}

pub(crate) fn sock_leave_multicast_v6(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    multiaddr: WasmPtr<__wasi_addr_ip6_t, MemoryType>,
    iface: u32,
) -> Errno {
    super::sock_leave_multicast_v6::<MemoryType>(ctx, sock, multiaddr, iface)
}

pub(crate) fn sock_bind(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    addr: WasmPtr<__wasi_addr_port_t, MemoryType>,
) -> Errno {
    super::sock_bind::<MemoryType>(ctx, sock, addr)
}

pub(crate) fn sock_listen(ctx: FunctionEnvMut<WasiEnv>, sock: Fd, backlog: MemoryOffset) -> Errno {
    super::sock_listen::<MemoryType>(ctx, sock, backlog)
}

pub(crate) fn sock_accept(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    fd_flags: Fdflags,
    ro_fd: WasmPtr<Fd, MemoryType>,
    ro_addr: WasmPtr<__wasi_addr_port_t, MemoryType>,
) -> Result<Errno, WasiError> {
    super::sock_accept::<MemoryType>(ctx, sock, fd_flags, ro_fd, ro_addr)
}

pub(crate) fn sock_connect(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    addr: WasmPtr<__wasi_addr_port_t, MemoryType>,
) -> Errno {
    super::sock_connect::<MemoryType>(ctx, sock, addr)
}

pub(crate) fn sock_recv(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    ri_data: WasmPtr<__wasi_iovec_t<MemoryType>, MemoryType>,
    ri_data_len: MemoryOffset,
    ri_flags: __wasi_riflags_t,
    ro_data_len: WasmPtr<MemoryOffset, MemoryType>,
    ro_flags: WasmPtr<__wasi_roflags_t, MemoryType>,
) -> Result<Errno, WasiError> {
    super::sock_recv::<MemoryType>(
        ctx,
        sock,
        ri_data,
        ri_data_len,
        ri_flags,
        ro_data_len,
        ro_flags,
    )
}

pub(crate) fn sock_recv_from(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    ri_data: WasmPtr<__wasi_iovec_t<MemoryType>, MemoryType>,
    ri_data_len: MemoryOffset,
    ri_flags: __wasi_riflags_t,
    ro_data_len: WasmPtr<MemoryOffset, MemoryType>,
    ro_flags: WasmPtr<__wasi_roflags_t, MemoryType>,
    ro_addr: WasmPtr<__wasi_addr_port_t, MemoryType>,
) -> Result<Errno, WasiError> {
    super::sock_recv_from::<MemoryType>(
        ctx,
        sock,
        ri_data,
        ri_data_len,
        ri_flags,
        ro_data_len,
        ro_flags,
        ro_addr,
    )
}

pub(crate) fn sock_send(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    si_data: WasmPtr<__wasi_ciovec_t<MemoryType>, MemoryType>,
    si_data_len: MemoryOffset,
    si_flags: __wasi_siflags_t,
    ret_data_len: WasmPtr<MemoryOffset, MemoryType>,
) -> Result<Errno, WasiError> {
    super::sock_send::<MemoryType>(ctx, sock, si_data, si_data_len, si_flags, ret_data_len)
}

pub(crate) fn sock_send_to(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    si_data: WasmPtr<__wasi_ciovec_t<MemoryType>, MemoryType>,
    si_data_len: MemoryOffset,
    si_flags: __wasi_siflags_t,
    addr: WasmPtr<__wasi_addr_port_t, MemoryType>,
    ret_data_len: WasmPtr<MemoryOffset, MemoryType>,
) -> Result<Errno, WasiError> {
    super::sock_send_to::<MemoryType>(
        ctx,
        sock,
        si_data,
        si_data_len,
        si_flags,
        addr,
        ret_data_len,
    )
}

pub(crate) fn sock_send_file(
    ctx: FunctionEnvMut<WasiEnv>,
    out_fd: Fd,
    in_fd: Fd,
    offset: __wasi_filesize_t,
    count: __wasi_filesize_t,
    ret_sent: WasmPtr<__wasi_filesize_t, MemoryType>,
) -> Result<Errno, WasiError> {
    unsafe { super::sock_send_file::<MemoryType>(ctx, out_fd, in_fd, offset, count, ret_sent) }
}

pub(crate) fn sock_shutdown(
    ctx: FunctionEnvMut<WasiEnv>,
    sock: Fd,
    how: __wasi_sdflags_t,
) -> Errno {
    super::sock_shutdown(ctx, sock, how)
}

pub(crate) fn resolve(
    ctx: FunctionEnvMut<WasiEnv>,
    host: WasmPtr<u8, MemoryType>,
    host_len: MemoryOffset,
    port: u16,
    ips: WasmPtr<__wasi_addr_t, MemoryType>,
    nips: MemoryOffset,
    ret_nips: WasmPtr<MemoryOffset, MemoryType>,
) -> Errno {
    super::resolve::<MemoryType>(ctx, host, host_len, port, ips, nips, ret_nips)
}
