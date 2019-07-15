for f in `ls | egrep "tellene.*\.png"`; do
	old_x=`echo $f | sed -r 's/tellene_([0-9]+)_([0-9]+)\.png/\1/'`
	old_y=`echo $f | sed -r 's/tellene_([0-9]+)_([0-9]+)\.png/\2/'`
	background="majenta"

	lat=`bc -l <<< "53 - $old_x"`
	long=`bc -l <<< "$old_y - 26"`
	new_filename="new_${lat}_${long}.png"

	if [[ "$old_x" -eq "0" ]]; then
		# North edge
		if [[ "$old_y" -eq "0" ]]; then
			# Northwest corner
			convert $f -background "$background" -gravity southeast -extent 512x512 $new_filename
		elif [[ "$old_y" -eq "38" ]]; then
			# Northeast corner
			convert $f -background "$background" -gravity southwest -extent 512x512 $new_filename
		else
			# Middle of north edge
			convert $f -resize 512! -background "$background" -gravity south -extent 512x512 $new_filename
		fi
	elif [[ "$old_x" -eq "30" ]]; then
		# South edge
		if [[ "$old_y" -eq "0" ]]; then
			# Southwest corner
			convert $f -background "$background" -gravity northeast -extent 512x512 $new_filename
		elif [[ "$old_y" -eq "38" ]]; then
			# Southeast corner
			convert $f -background "$background" -gravity northwest -extent 512x512 $new_filename
		else
			# Middle of south edge
			convert $f -resize 512! -background "$background" -gravity north -extent 512x512 $new_filename
		fi
	else
		# In the middle, north/south wise
		if [[ "$old_y" -eq "0" ]]; then
			# Middle of west edge
			convert $f -resize x512! -background "$background" -gravity east -extent 512x512 $new_filename
		elif [[ "$old_y" -eq "38" ]]; then
			# Middle of east edge
			convert $f -resize x512! -background "$background" -gravity west -extent 512x512 $new_filename
		else
			# Middle of image
			convert $f -resize 512x512! $new_filename
		fi
	fi

	rm $f
done

for f in `ls | egrep "new_-?[0-9]+_-?[0-9]+\.png"`; do
	mv $f `echo $f | sed -r 's/new(.*)/tellene\1/'`
done
