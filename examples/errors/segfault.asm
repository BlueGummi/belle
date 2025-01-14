.start [30]
.ssp [10]
.sbp [10]
     ld r0, $300 ; This causes a segmentation fault as it loads from an empty address.
     hlt
