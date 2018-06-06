#!/bin/bash
set -eu

# You should install `ggplot2` and `reshape2` using R.

mine=$(mktemp /tmp/save-zsh-history.XXXXXX)
cat <<-'_EOM_'>"$mine"
#!/usr/bin/env perl
use strict;
use warnings;
use autodie;
use feature qw(say);

open(my $fh, '-|', 'git log --shortstat')
  or die "cannot pipe from `git log`: $!";

my $stack;
my ($month, $day, $year);
while (my $line = <$fh>) {
  chomp $line;
  if ($line =~ /^Date:\s+(?:...)\s(...)\s(\d+)\s(?:\d\d:\d\d:\d\d)\s(\d\d\d\d)/o) {
    ($month, $day, $year) = ($1, $2, $3);
  } elsif ($line =~ /^\s+\d+ files? changed/o) {
    my $insertions = ($line =~ /(\d+) insertions\(\+\)/o) ? $1 : 0;
    my $deletions = ($line =~ /(\d+) deletions\(\-\)/o) ? $1 : 0;
    push @$stack, join(',', "$year $month $day", $deletions, $insertions);
  }
}

say 'date,deletions,insertions';
for my $e (reverse @$stack) {
  say $e;
}
_EOM_

plot=$(mktemp /tmp/save-zsh-history.XXXXXX)
cat <<-'_EOM_'>"$plot"
suppressPackageStartupMessages(library(ggplot2))
suppressPackageStartupMessages(library(reshape2))

pdf('plot.pdf')
d <- read.csv('./mined.csv')
d <- melt(d)
d$date <- factor(d$date, ordered = T)
d$date <- as.Date(d$date, format="%Y %b %d")
g <- ggplot(d, aes(x = as.character(date), y = value, fill = variable))
plot(g + geom_bar(stat = 'identity')
       + labs(list(x = 'date', y = '# of lines'))
       + theme(text = element_text(family = 'ArialMT'))
       + coord_flip()
       + guides(fill = guide_legend(reverse = T)))
graphics.off()
_EOM_

perl "$mine" >"mined.csv" && Rscript --vanilla "$plot" && rm -f "$mine" "mined.csv" "$plot"
