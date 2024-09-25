import os


f = open("/Users/jpsteinb/github.com/Devland-agancy/little-bo-peep-leptos/src/content/chapter3/chapter.emu")


filenames_to_rename = []

for l in f.readlines():
    if "src images/" in l:
        filename = l.split("src images/")[1]
        filename = filename.strip()
        assert "_ch3_" in filename
        pieces = filename.split('_ch3')
        reassembled = pieces[0] + pieces[1]
        filenames_to_rename.append((reassembled, filename))


path = "/Users/jpsteinb/github.com/Devland-agancy/little-bo-peep-leptos/public/images/"
 
for (key, value) in filenames_to_rename:
    # print(key)
    # print(value)
    
    orig = path + key
    dest = path + value
    
    try:
        os.rename(orig, dest)
        
    except:
        print(f"couldn't rename: {orig}")
