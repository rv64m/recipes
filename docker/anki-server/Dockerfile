FROM python:3.12-slim

ARG ANKI_VERSION=25.9.4

ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    SYNC_HOST=0.0.0.0 \
    SYNC_PORT=8080 \
    SYNC_BASE=/data

RUN python -m pip install --no-cache-dir --upgrade pip \
    && python -m pip install --no-cache-dir "anki==${ANKI_VERSION}" \
    && useradd --system --uid 10001 --create-home --home-dir /home/anki anki \
    && mkdir -p /data \
    && chown -R anki:anki /data /home/anki

USER anki

WORKDIR /data
VOLUME ["/data"]
EXPOSE 8080

CMD ["python", "-m", "anki.syncserver"]
