# Tutorial: download a video from a page

[← Back to docs index](README.md)

Save a video from YouTube, Vimeo, TikTok and many other sites, in the quality you choose.

## Steps

1. Open the video in your browser and copy the page's address from the address bar.
2. In MDM, click **Add URL** (or press **Ctrl+N**) and click **OK**. MDM sees that the address is a video's page and opens **Download video from a page**. (You can also open it with **Tasks → Download video from a page...**, paste the address and click **Look up**.)
3. The first time, MDM needs its video helper. Click **Install yt-dlp** (about 23 MB). MDM downloads it from its official release and checks it before use.
4. Wait while MDM looks the page up. The list of qualities appears, best first, with **Audio only** at the end: the sound as the site has it, then, with ffmpeg (next step), as an MP3 at its best variable bitrate or at 320, 256, 192 or 128 kbps.
5. For the best qualities and MP3, MDM needs **ffmpeg**, which joins the picture and the sound, and makes the MP3. On Windows MDM offers to install it (about 80 MB): click **Install ffmpeg** if you want them. On Linux and macOS MDM uses the system's ffmpeg. Without it, you get the qualities that come with their sound.
6. Pick a quality.
7. Check **Save to** and **File name**. The name comes from the video's title; change it if you like.
8. Want subtitles? Pick a language in **Subtitles**. They're saved beside the video, or inside it if you tick **Put them into the video** (see [Subtitles](video.md#subtitles)).
9. Click **Download Now**, or **Later** to add it to the list without starting.

The video appears in the list and downloads like any other file.

<!-- steps -->

1. ![The video window listing a video's qualities with their sizes](images/help/vfp-qualities.webp)
   *MDM lists the qualities: pick one.*
2. ![A video's progress window](images/help/vfp-progress.webp)
   *It downloads like any other file.*

## Good to know

- A link to one video inside a playlist or a mix downloads just that video, not the whole list.
- Videos protected with DRM (most paid streaming services) can't be downloaded.
- Downloading may be against some sites' terms; that is up to you to check.

If the video won't download, see [Video downloads](video.md#if-a-video-wont-download).

## Next steps

- [Video downloads in detail →](video.md)
- [Find a download in a long list →](guide-find-in-long-list.md)
- [Back to the docs index →](README.md)

## A downloaded video plays with the sound only

Some players show a downloaded video as sound with no picture. Usually the file is not broken: it
is **AV1** (`av01`), and that player has no AV1 decoder. YouTube serves **2160p (4K) and above only
as AV1 or VP9** — there is no H.264 file at that size — so a 4K download from YouTube is AV1, while
1080p and below are H.264/MP4 that every player opens.

How to play it:

- Play the file in **VLC, MPC-HC or PotPlayer**. All three are free and decode AV1 in software.
- Or install Microsoft's free **AV1 Video Extension** from the Microsoft Store (search for "AV1
  Video Extension"); players that use Windows' own media codecs then open AV1 too.
- Or update your player: **GOM Player** and **Windows Media Player** need a recent version with AV1
  support.
- Or download the **1080p** version of the video instead: in {{name}}'s quality list a video choice
  shows its codec beside its height ("2160p · AV1" or "1080p · H.264"), so pick the H.264 line when
  the player is old.

Converting the 4K AV1 file to H.264 afterwards is possible with ffmpeg (`ffmpeg -i in.mp4 -c:v
libx264 -crf 20 out.mp4`), but it takes hours for a two-hour video and loses a little quality.
