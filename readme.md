# winamp/wacup yule logs to listenbrainz 

Small rust script to add winamp logged music to listenbrainz

## Usage

currently requires running local listenbrainz server, this will bet changed in the release

run the script with the first argument being the folder location of all the logs, the second being your listenbrainz token which the files will be submitted to.

## Edge cases

most edge cases just give a warning in the console and are ignored, as this program isnt intended to perfectly upload all data, just to help fill out some of the possible gaps.

- most, if not all incorrectly tagged music breaks and does not get sent (it does give a warning though)
- over 1 hour songs are not supported and ignored 
- listening over 2 days, just ignores the last songs playtime



## Later

maybe make it a direct plugin to winamp instead of just reading the log files
