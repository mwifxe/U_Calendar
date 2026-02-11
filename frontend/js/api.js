const API = {
    BASE: '/api',

    async getCalendario() {
        const res = await fetch(`${this.BASE}/calendario`);
        if (!res.ok) throw new Error(`GET /calendario failed: ${res.status}`);
        return res.json();
    },

    async createActividad(semanaId, data) {
        const res = await fetch(`${this.BASE}/semanas/${semanaId}/actividades`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(data),
        });
        if (!res.ok) {
            const err = await res.json().catch(() => ({}));
            throw new Error(err.error || `POST actividad failed: ${res.status}`);
        }
        return res.json();
    },

    async updateActividad(actividadId, data) {
        const res = await fetch(`${this.BASE}/actividades/${actividadId}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(data),
        });
        if (!res.ok) {
            const err = await res.json().catch(() => ({}));
            throw new Error(err.error || `PUT actividad failed: ${res.status}`);
        }
        return res.json();
    },

    async deleteActividad(actividadId) {
        const res = await fetch(`${this.BASE}/actividades/${actividadId}`, {
            method: 'DELETE',
        });
        if (!res.ok && res.status !== 204) {
            throw new Error(`DELETE actividad failed: ${res.status}`);
        }
    },

    async getNota(actividadId) {
        const res = await fetch(`${this.BASE}/actividades/${actividadId}/notas`);
        if (!res.ok) throw new Error(`GET nota failed: ${res.status}`);
        return res.json();
    },

    async saveNota(actividadId, contenido) {
        const res = await fetch(`${this.BASE}/actividades/${actividadId}/notas`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ contenido }),
        });
        if (!res.ok) throw new Error(`PUT nota failed: ${res.status}`);
        return res.json();
    },
};