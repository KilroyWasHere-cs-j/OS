echo "Running build as debug..."

cargo build

echo "Cargo build complete"

echo "Createing boot image..."

cargo bootimage

echo "Booting OS..."
echo "Press Ctrl-a x to exit"
echo "Booting OS from /home/gabrielt/Git/OS/blog_os/target/x86_64-blog_os/debug/bootimage-blog_os.bin"


qemu-system-x86_64 -drive format=raw,file=/home/gabrielt/Git/OS/blog_os/target/x86_64-blog_os/debug/bootimage-blog_os.bin
