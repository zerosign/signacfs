# signacfs

Signature &amp; access control based overlay filesystem (supports for fuse, docker &amp; hopefully kubernetes using CRI)
This fuse filesystem being implemented since LSM like SELinux still aren't being supported in
namespace based system (container based).

This filesystem use access control + signature based to check whether incoming request from process
(gid, uid, pid) are allowed to a request to filesystem. This enable us to do a fine grain based
access to the filesystem so that we could potentially secure an access to specific files by specifying
certain policies to access control. At first it will check whether application signature (*.crt, *.ca.crt)
are valid or not then check the checksum of the signature valid or not.
