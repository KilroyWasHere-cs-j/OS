echo "Running build as debug..."

cargo build

echo "Cargo build complete"

echo "Createing boot image..."

while true; do

	read -p "Do you wish to create a boot image? [y/n]" yn

	case $yn in
	[Yy]*)
		cargo bootimage
		break
		;;
	[Nn]*) exit ;;
	*) echo "Please answer yes or no." ;;
	esac
done

echo "Booting OS..."
echo "Press Ctrl-a x to exit"
echo "Booting OS from /home/gabrielt/Git/OS/blog_os/target/x86_64-blog_os/debug/bootimage-blog_os.bin"

while true; do
	read -p "Do you wish to boot the OS? [y/n]" yn

	case $yn in
	[Yy]*)
		qemu-system-x86_64 -drive format=raw,file=/home/gabrielt/Git/OS/blog_os/target/x86_64-blog_os/debug/bootimage-blog_os.bin
		break
		;;
	[Nn]*) exit ;;
	*) echo "Please answer yes or no." ;;
	esac

done
