# winamp/wacup yule logs to listenbrainz 

Small rust script to add winamp logged music to listenbrainz

## prerequisites

from winamp/wacup, have logging playback history to file enabled. This can be found in preferences -> Playback -> Play tracking -> Playback History
the format should be `%date% - %time% - "%artist%" - "%title%" - %length%`

## Usage

run the script with the first argument being the folder location of all the logs, the second being your listenbrainz token for the account which the listens will be submitted to.

example usage:
`.\yule-to-brainz.exe C:\Users\username\AppData\Roaming\WACUP\Logs 1sdfr1dx-1906-4674-b041-f2f6405h32yf`

## Edge cases

most edge cases just give a warning in the console and are ignored, as this program isn't intended to perfectly upload all data, just to help fill out some of the possible gaps.

- any song with the string ` - ` gives out a warning and is ignored. 
- over 1 hour songs are not supported and ignored.
- listening over the dateline of 2 days just ignores the last songs playtime and submits it regardless.

There are probably other edge cases that cause errors with incorrect formatting. 
