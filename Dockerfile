
FROM scratch
ARG TARGETARCH

COPY dd-metric-$TARGETARCH/dd-metric /dd-metric

CMD ["/dd-metric"]
