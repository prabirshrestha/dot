set FNAME=%TARGET%-%HOST%

mkdir %FNAME%
copy target\release\%TARGET%.exe .\%FNAME%\%TARGET%.exe
copy completions\* .\%FNAME%\
