CREATE TABLE users (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE projects (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    status TEXT DEFAULT 'draft',
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE videos (
    id UUID PRIMARY KEY,
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    original_url TEXT NOT NULL,
    duration FLOAT,
    width INT,
    height INT,
    fps FLOAT,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE subtitle_tracks (
    id UUID PRIMARY KEY,
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    language TEXT DEFAULT 'en',
    name TEXT DEFAULT 'Auto Captions',
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE subtitle_segments (
    id UUID PRIMARY KEY,
    track_id UUID REFERENCES subtitle_tracks(id) ON DELETE CASCADE,
    start_time FLOAT NOT NULL,
    end_time FLOAT NOT NULL,
    text TEXT NOT NULL,
    style JSONB DEFAULT '{}'::jsonb
);

CREATE TABLE render_jobs (
    id UUID PRIMARY KEY,
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    status TEXT DEFAULT 'queued',
    progress FLOAT DEFAULT 0,
    input_video_url TEXT,
    output_video_url TEXT,
    error TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE transcription_jobs (
    id UUID PRIMARY KEY,
    video_id UUID REFERENCES videos(id) ON DELETE CASCADE,
    status TEXT DEFAULT 'queued',
    progress FLOAT DEFAULT 0,
    transcript_json JSONB DEFAULT '{}'::jsonb,
    error TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE assets (
    id UUID PRIMARY KEY,
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    type TEXT NOT NULL,
    url TEXT NOT NULL,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP DEFAULT NOW()
);