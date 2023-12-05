# Winamp/wacup yule logs to listenbrainz 

Small rust script to add winamp logged music to listenbrainz

## Prerequisites

From winamp/wacup, have logging playback history to file enabled. This can be found in preferences -> Playback -> Play tracking -> Playback History

The format should be `%date% - %time% - "%artist%" - "%title%" - %length%`

## Usage

Run the script with the first argument being the folder location of all the logs, the second being your listenbrainz token for the account which the listens will be submitted to.

Example usage:
`.\yule-to-brainz.exe C:\Users\username\AppData\Roaming\WACUP\Logs 1sdfr1dx-1906-4674-b041-f2f6405h32yf`

The script does delete your log files, possibly adding an option to change this behaviour later.

## Edge cases

Most edge cases just give a warning in the console and are ignored, as this program isn't intended to perfectly upload all data, just to help fill out some of the possible gaps.

- Any song name with the string `" - "` gives out a warning and is ignored. 
- Over 1 hour songs are not supported and ignored.
- Listening over the dateline of 2 days just ignores the last songs playtime and submits it regardless.

There are probably other edge cases that cause errors with incorrect formatting. It is what it is.
